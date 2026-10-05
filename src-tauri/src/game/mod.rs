use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    hash::{self, Hash},
    sync::LazyLock,
};

use chrono::{DateTime, Utc};
use eyre::{OptionExt, Result, ensure};
use heck::{ToKebabCase, ToPascalCase};
use serde::{Deserialize, Serialize};

use mod_loader::ModLoader;
use platform::Platforms;
use tauri::AppHandle;
use tracing::{info, warn};

use crate::{
    state::ManagerExt,
    thunderstore::Backend,
    util::{self, fs::JsonStyle},
};

pub mod mod_loader;
pub mod platform;

// not Gale's games.json: an older Crosswind cached the list with every game in it
pub const CACHE_FILE_NAME: &str = "mac-games.json";

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/Kesomannen/gale/commits?path=src-tauri/games.json&per_page=1";
const GAMES_JSON_URL: &str =
    "https://raw.githubusercontent.com/Kesomannen/gale/refs/heads/master/src-tauri/games.json";
const STEAM_ITEMS_URL: &str = "https://api.steampowered.com/IStoreBrowseService/GetItems/v1/";

const BUNDLED_GAMES_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/games.json"));

const BUILD_TIME: &str = env!("BUILD_TIME");

static GAMES: LazyLock<(DateTime<Utc>, Vec<GameData<'static>>)> = LazyLock::new(|| {
    if !cfg!(debug_assertions) {
        match get_cached_games() {
            Ok(cache) => {
                info!("using cached games list, last commit at {}", cache.date);
                return (cache.date, cache.games);
            }
            Err(err) => {
                warn!("failed to read cached games list: {err}");
            }
        }
    }

    let updated_at = DateTime::parse_from_rfc3339(BUILD_TIME).unwrap().to_utc();

    info!("using bundled games list (built at {updated_at})");

    (
        updated_at,
        serde_json::from_str(BUNDLED_GAMES_JSON).unwrap(),
    )
});

#[derive(Debug, Serialize, Deserialize)]
struct GamesCache<'a> {
    date: DateTime<Utc>,
    #[serde(borrow)]
    games: Vec<GameData<'a>>,
}

fn get_cached_games() -> Result<GamesCache<'static>> {
    let path = util::path::default_app_data_dir().join(CACHE_FILE_NAME);

    let str = fs::read_to_string(path)?;
    let games = serde_json::from_str(str.leak())?;

    Ok(games)
}

pub async fn update_list_task(app: &AppHandle) -> Result<()> {
    if cfg!(debug_assertions) {
        info!("skipping games list update in debug mode");
        return Ok(());
    }

    let str = app
        .http()
        .get(GAMES_JSON_URL)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let games = mac_only(app.http(), serde_json::from_str(&str)?).await?;

    let date = get_last_commit_date(app).await.unwrap_or_else(|err| {
        warn!("failed to get last commit date: {err}");
        Utc::now()
    });

    let cache = GamesCache { date, games };

    let path = util::path::default_app_data_dir().join(CACHE_FILE_NAME);
    util::fs::write_json(path, &cache, JsonStyle::Pretty)?;

    info!("updated games list from github, last commit at {date}");

    Ok(())
}

/// The games whose Steam store page lists macOS: Crosswind is a Mac mod manager. Gale's list has
/// every game Thunderstore supports.
async fn mac_only<'a>(
    http: &reqwest_middleware::ClientWithMiddleware,
    games: Vec<GameData<'a>>,
) -> Result<Vec<GameData<'a>>> {
    let ids: Vec<_> = games
        .iter()
        .filter_map(|game| game.platforms.steam.as_ref())
        .map(|steam| serde_json::json!({ "appid": steam.id }))
        .collect();
    let input = serde_json::json!({
        "ids": ids,
        "context": { "language": "english", "country_code": "US" },
        "data_request": { "include_platforms": true },
    });

    let url =
        reqwest::Url::parse_with_params(STEAM_ITEMS_URL, [("input_json", input.to_string())])?;
    let response: serde_json::Value = http
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mac: HashSet<u64> = response["response"]["store_items"]
        .as_array()
        .ok_or_eyre("unexpected response from Steam")?
        .iter()
        .filter(|item| item["platforms"]["mac"] == true)
        .filter_map(|item| item["appid"].as_u64())
        .collect();
    ensure!(!mac.is_empty(), "Steam listed no Mac games");

    Ok(games
        .into_iter()
        .filter(|game| {
            game.platforms
                .steam
                .as_ref()
                .is_some_and(|steam| mac.contains(&u64::from(steam.id)))
        })
        .collect())
}

async fn get_last_commit_date(app: &AppHandle) -> Result<DateTime<Utc>> {
    #[derive(Debug, Deserialize)]
    struct ResponseEntry {
        commit: Commit,
    }

    #[derive(Debug, Deserialize)]
    struct Commit {
        author: Author,
    }

    #[derive(Debug, Deserialize)]
    struct Author {
        date: DateTime<Utc>,
    }

    let response: Vec<ResponseEntry> = app
        .http()
        .get(GITHUB_API_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let date = response
        .first()
        .ok_or_eyre("github api response contained no entries")?
        .commit
        .author
        .date;

    Ok(date)
}

pub type Game = &'static GameData<'static>;

pub fn list() -> impl Iterator<Item = Game> {
    GAMES.1.iter()
}

pub fn from_slug(slug: &str) -> Option<Game> {
    GAMES.1.iter().find(|game| game.slug == slug)
}

pub fn last_updated() -> DateTime<Utc> {
    GAMES.0
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonGame<'a> {
    name: &'a str,

    #[serde(default)]
    slug: Option<&'a str>,

    #[serde(default)]
    popular: bool,

    #[serde(default)]
    server: bool,

    #[serde(default, rename = "r2dirName")]
    r2_dir_name: Option<&'a str>,

    #[serde(borrow)]
    mod_loader: ModLoader<'a>,

    #[serde(borrow, default)]
    platforms: Platforms<'a>,

    #[serde(default)]
    backends: Option<Vec<Backend>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", from = "JsonGame")]
pub struct GameData<'a> {
    pub name: &'a str,
    pub slug: Cow<'a, str>,
    pub r2_dir_name: Cow<'a, str>,
    pub popular: bool,
    pub server: bool,
    pub mod_loader: ModLoader<'a>,
    pub platforms: Platforms<'a>,
    pub backends: Vec<Backend>,
}

impl<'a> From<JsonGame<'a>> for GameData<'a> {
    fn from(value: JsonGame<'a>) -> Self {
        let JsonGame {
            name,
            slug,
            popular,
            server,
            r2_dir_name,
            mod_loader,
            platforms,
            backends,
        } = value;

        let slug = match slug {
            Some(slug) => Cow::Borrowed(slug),
            None => Cow::Owned(name.to_kebab_case()),
        };

        let r2_dir_name = match r2_dir_name {
            Some(name) => Cow::Borrowed(name),
            None => Cow::Owned(slug.to_pascal_case()),
        };

        Self {
            name,
            slug,
            r2_dir_name,
            popular,
            server,
            mod_loader,
            platforms,
            backends: backends.unwrap_or(vec![Backend::Thunderstore]),
        }
    }
}

impl PartialEq for GameData<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

impl Eq for GameData<'_> {}

impl Hash for GameData<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.slug.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rewrites the bundled games.json (used until the first update) from Gale's current list:
    /// `cargo test --lib write_bundled_games -- --ignored`
    #[tokio::test]
    #[ignore]
    async fn write_bundled_games() {
        let http = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let text = http
            .get(GAMES_JSON_URL)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mac: HashSet<_> = mac_only(&http, serde_json::from_str(&text).unwrap())
            .await
            .unwrap()
            .into_iter()
            .map(|game| game.name)
            .collect();
        // in Gale's own format
        let games: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap();
        let games: Vec<_> = games
            .into_iter()
            .filter(|game| game["name"].as_str().is_some_and(|name| mac.contains(name)))
            .collect();

        // tabs, like Gale's
        let mut json = Vec::new();
        let format = serde_json::ser::PrettyFormatter::with_indent(b"\t");
        games
            .serialize(&mut serde_json::Serializer::with_formatter(
                &mut json, format,
            ))
            .unwrap();
        json.push(b'\n');
        fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/games.json"), json).unwrap();
        println!("{} Mac games", games.len());
    }
}
