//! ROUNDS on the current game build.
//!
//! The December 2025 update broke every mod, and Thunderstore still serves the old UnboundLib 3 and
//! RoundsWithFriends 2 that most mods depend on. Before a modded launch this brings the profile's files
//! up to date, leaving the installed packages (and so dependency resolution) as they are:
//!
//! - the old UnboundLib, MMHook and RoundsWithFriends files are swapped for Bknibb's ports, downloaded
//!   from GitHub and checked by SHA-256;
//! - exact mod versions that need hand-made fixes get the rounds-mac-modpack's binary patches;
//! - BepInEx's `HideManagerGameObject` is turned on (otherwise the game destroys plugin objects);
//! - rounds-port AutoFix goes into patchers: it fixes the other old mods while BepInEx starts;
//! - Mac Compat Fixes and the Odin Serializer stand-in go into plugins.
//!
//! Files are replaced, never written through, because profile files are hard links into Gale's cache.

use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use eyre::{Context, Result, bail, ensure};
use include_dir::{Dir, include_dir};
use sha2::{Digest, Sha256};
use tauri::AppHandle;
use tracing::{info, warn};
use walkdir::WalkDir;

use crate::{state::ManagerExt, util};

pub const SLUG: &str = "rounds";

static FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/rounds");

/// A file from a GitHub release, pinned by hash.
struct Download {
    name: &'static str,
    url: &'static str,
    sha256: &'static str,
}

const UNBOUNDLIB: Download = Download {
    name: "UnboundLib.dll",
    url: "https://github.com/Bknibb/UnboundLib/releases/download/v4.2.5/UnboundLib.dll",
    sha256: "3411ae8451f7ad2bc7a4bf1b2e5fe21a4581afff44619f8a34b03430f3d4f408",
};
const OCTOKIT: Download = Download {
    name: "Octokit.dll",
    url: "https://github.com/Bknibb/UnboundLib/releases/download/v4.2.5/Octokit.dll",
    sha256: "6a48642d6ae464b43a6cb50292618af6c73ada8d726644e50d4fa44a1783f638",
};
const MMHOOK: Download = Download {
    name: "MMHOOK_Assembly-CSharp.dll",
    url: "https://github.com/Bknibb/UnboundLib/releases/download/v4.2.5/MMHOOK_Assembly-CSharp.dll",
    sha256: "926b53b329d94f6a8842e6d51ca17ff96f081df59695d7862845d5ccce9e5a62",
};
const RWF: Download = Download {
    name: "RoundsWithFriends.dll",
    url: "https://github.com/Bknibb/RoundsWithFriends/releases/download/v3.0.10/RoundsWithFriends.dll",
    sha256: "1bd4d5aa47de0e04661710a77bb0b5f1214dac4b3baabc9364b3418ecbc8ab61",
};

enum Source {
    Download(&'static Download),
    /// A file shipped with Gale, under `resources/rounds/files`.
    Bundled(&'static str, &'static str),
}

impl Source {
    fn name(&self) -> &'static str {
        match self {
            Source::Download(download) => download.name,
            Source::Bundled(name, _) => name,
        }
    }
}

/// Files put into a package's folder in `BepInEx/plugins` when that package is installed.
const PACKAGE_FILES: &[(&[&str], &[Source])] = &[
    (
        &["willis81808-UnboundLib", "olavim-UnboundLib"],
        &[Source::Download(&UNBOUNDLIB), Source::Download(&OCTOKIT)],
    ),
    (&["willis81808-MMHook"], &[Source::Download(&MMHOOK)]),
    (&["olavim-RoundsWithFriends"], &[Source::Download(&RWF)]),
];

const ODIN: &str = "BepInEx/plugins/OdinSerializer";

/// Files every ROUNDS profile gets, relative to the profile.
const PROFILE_FILES: &[(&str, Source)] = &[
    // rounds-port AutoFix: fixes the remaining old mods while BepInEx starts
    (
        "BepInEx/patchers/RoundsPort-AutoFix",
        Source::Bundled("rounds-port.AutoFix.dll", "rounds-port.AutoFix.dll"),
    ),
    (
        "BepInEx/plugins/RoundsMacModpack-MacCompatFixes",
        Source::Bundled("MacCompatFixes.dll", "MacCompatFixes.dll"),
    ),
    // The game no longer ships Odin Serializer; MapsExtended, WillsWackyCards and others use it.
    // This is the Apache-2.0 open-source version.
    (ODIN, Source::Bundled("Sirenix.Serialization.dll", "odin/Sirenix.Serialization.dll")),
    (ODIN, Source::Bundled("Sirenix.Serialization.Config.dll", "odin/Sirenix.Serialization.Config.dll")),
    (ODIN, Source::Bundled("Sirenix.Utilities.dll", "odin/Sirenix.Utilities.dll")),
    (ODIN, Source::Bundled("LICENSE.txt", "odin/Sirenix-OdinSerializer-LICENSE.txt")),
];

/// Folders earlier builds of this layer installed, removed from profiles.
const REMOVED_DIRS: &[&str] = &[
    "BepInEx/plugins/RoundsMacModpack-MacCompatFixes",
    "BepInEx/plugins/RoundsMacModpack-OdinSerializer",
];

/// A binary patch from the rounds-mac-modpack, for one exact file.
struct Patch {
    name: String,
    before: String,
    after: String,
    patch: String,
}

fn patches() -> Vec<Patch> {
    let tsv = FILES
        .get_file("patches.tsv")
        .and_then(|file| file.contents_utf8())
        .unwrap_or_default();

    tsv.lines()
        .filter_map(|line| {
            let mut cols = line.split('\t');
            let path = cols.next()?;
            Some(Patch {
                name: path.rsplit('/').next()?.to_string(),
                before: cols.next()?.to_string(),
                after: cols.next()?.to_string(),
                patch: cols.next()?.to_string(),
            })
        })
        .collect()
}

/// Brings the active ROUNDS profile up to date. Does nothing for other games.
pub async fn prepare(app: &AppHandle) -> Result<()> {
    let profile_dir = {
        let manager = app.lock_manager();
        let game = manager.active_game();
        if game.game.slug != SLUG {
            return Ok(());
        }
        manager.active_profile().path.clone()
    };

    update_profile(&profile_dir, app.http()).await.map(|_| ())
}

/// Returns what changed.
async fn update_profile(
    profile_dir: &Path,
    http: &reqwest_middleware::ClientWithMiddleware,
) -> Result<Vec<String>> {

    let plugins = profile_dir.join("BepInEx/plugins");
    let packages: Vec<_> = PACKAGE_FILES
        .iter()
        .filter(|(names, _)| names.iter().any(|name| plugins.join(name).is_dir()))
        .collect();

    // Download what's missing first, so nothing is changed if a download fails.
    let mut downloaded = HashMap::new();
    let needed = packages
        .iter()
        .flat_map(|(_, sources)| sources.iter())
        .chain(PROFILE_FILES.iter().map(|(_, source)| source));
    for source in needed {
        if let Source::Download(download) = source
            && !downloaded.contains_key(download.name)
        {
            let bytes = fetch(http, download).await?;
            downloaded.insert(download.name, bytes);
        }
    }

    let patches = patches();
    let mut changed = Vec::new();

    for dir in REMOVED_DIRS {
        let path = profile_dir.join(dir);
        if path.is_dir() {
            fs::remove_dir_all(&path).with_context(|| format!("failed to remove {}", path.display()))?;
            changed.push(format!("removed {dir}"));
        }
    }

    for (names, sources) in packages {
        for name in names.iter().filter(|name| plugins.join(name).is_dir()) {
            for source in sources.iter() {
                let path = plugins.join(name).join(source.name());
                let bytes = contents(source, &downloaded)?;
                if put(&path, bytes, &patches)? {
                    changed.push(format!("{name}/{}", source.name()));
                }
            }
        }
    }

    for (dir, source) in PROFILE_FILES {
        let path = profile_dir.join(dir).join(source.name());
        if put(&path, contents(source, &downloaded)?, &patches)? {
            changed.push(format!("{dir}/{}", source.name()));
        }
    }

    for path in plugin_files(&plugins) {
        match apply_patch(&path, &patches) {
            Ok(true) => changed.push(format!("patched {}", path.display())),
            Ok(false) => (),
            Err(err) => warn!("failed to patch {}: {err:#}", path.display()),
        }
    }

    if set_hide_manager(&profile_dir)? {
        changed.push("BepInEx.cfg: HideManagerGameObject = true".into());
    }

    if changed.is_empty() {
        info!("ROUNDS profile is up to date");
    } else {
        info!("updated the ROUNDS profile:\n  {}", changed.join("\n  "));
    }

    Ok(changed)
}

fn contents<'a>(source: &Source, downloaded: &'a HashMap<&str, Vec<u8>>) -> Result<&'a [u8]> {
    match source {
        Source::Download(download) => Ok(&downloaded[download.name]),
        Source::Bundled(_, path) => match FILES.get_file(format!("files/{path}")) {
            Some(file) => Ok(file.contents()),
            None => bail!("{path} is missing from the app"),
        },
    }
}

/// Puts `bytes` at `path` unless the file there already is that file, or that file with its patch
/// applied. Returns whether anything was written.
fn put(path: &Path, bytes: &[u8], patches: &[Patch]) -> Result<bool> {
    let want = sha256(bytes);
    let patched = patches.iter().find(|patch| patch.before == want).map(|patch| &patch.after);

    if let Ok(current) = fs::read(path) {
        let current = sha256(&current);
        if current == want || Some(&current) == patched {
            return Ok(false);
        }
    }

    replace(path, bytes)?;
    Ok(true)
}

/// Applies the matching patch to a file, if there is one. Returns whether it did.
fn apply_patch(path: &Path, patches: &[Patch]) -> Result<bool> {
    let name = util::fs::file_name_owned(path);
    let candidates: Vec<_> = patches.iter().filter(|patch| patch.name == name).collect();
    if candidates.is_empty() {
        return Ok(false);
    }

    let source = fs::read(path)?;
    let hash = sha256(&source);
    let Some(patch) = candidates.into_iter().find(|patch| patch.before == hash) else {
        return Ok(false);
    };

    let diff = FILES
        .get_file(format!("patches/{}", patch.patch))
        .map(|file| file.contents())
        .ok_or_else(|| eyre::eyre!("{} is missing from the app", patch.patch))?;

    let mut target = Vec::new();
    qbsdiff::Bspatch::new(diff)
        .context("invalid patch")?
        .apply(&source, std::io::Cursor::new(&mut target))
        .context("failed to apply patch")?;

    ensure!(sha256(&target) == patch.after, "patch result has the wrong hash");

    replace(path, &target)?;
    Ok(true)
}

fn plugin_files(plugins: &Path) -> Vec<PathBuf> {
    WalkDir::new(plugins)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .collect()
}

/// Sets `HideManagerGameObject = true` in BepInEx.cfg. Returns whether the file changed.
fn set_hide_manager(profile_dir: &Path) -> Result<bool> {
    let path = profile_dir.join("BepInEx/config/BepInEx.cfg");

    let text = fs::read_to_string(&path).unwrap_or_default();
    let mut found = false;
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("HideManagerGameObject") {
                found = true;
                "HideManagerGameObject = true".to_string()
            } else {
                line.to_string()
            }
        })
        .collect();

    if !found {
        // BepInEx adds the rest of the file on its first start.
        lines.extend(["".into(), "[Chainloader]".into(), "HideManagerGameObject = true".into()]);
    }

    let mut new = lines.join("\n");
    new.push('\n');
    if new == text {
        return Ok(false);
    }

    replace(&path, new.as_bytes())?;
    Ok(true)
}

/// Writes a file next to `path` and renames it over, so a hard link at `path` is replaced rather
/// than written through.
fn replace(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path.parent().ok_or_else(|| eyre::eyre!("{} has no parent", path.display()))?;
    fs::create_dir_all(dir)?;

    let tmp = dir.join(format!(".{}.gale-tmp", util::fs::file_name_owned(path)));
    fs::write(&tmp, bytes).with_context(|| format!("failed to write {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("failed to replace {}", path.display()))?;

    Ok(())
}

/// A download from the cache, or from the network (then cached).
async fn fetch(
    http: &reqwest_middleware::ClientWithMiddleware,
    download: &Download,
) -> Result<Vec<u8>> {
    let cache = util::path::default_app_data_dir().join("rounds-cache").join(download.sha256);

    if let Ok(mut file) = fs::File::open(&cache) {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        if sha256(&bytes) == download.sha256 {
            return Ok(bytes);
        }
    }

    info!("downloading {}", download.url);

    let bytes = http
        .get(download.url)
        .send()
        .await
        .and_then(|response| response.error_for_status().map_err(Into::into))
        .with_context(|| format!("failed to download {}", download.url))?
        .bytes()
        .await
        .with_context(|| format!("failed to download {}", download.url))?
        .to_vec();

    ensure!(
        sha256(&bytes) == download.sha256,
        "{} doesn't match its expected SHA-256",
        download.url
    );

    if let Some(dir) = cache.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&cache, &bytes)?;

    Ok(bytes)
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Brings any profile folder up to date, for the in-game test bench:
    /// `ROUNDS_PREPARE_DIR=<dir> cargo test --lib prepare_dir -- --ignored`
    #[tokio::test]
    #[ignore]
    async fn prepare_dir() {
        let dir = PathBuf::from(std::env::var("ROUNDS_PREPARE_DIR").expect("ROUNDS_PREPARE_DIR"));
        let http = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let changed = update_profile(&dir, &http).await.unwrap();
        println!("{} changes", changed.len());
    }

    /// Runs against a profile made by `scripts/rounds-test-profile.sh` (needs the network once):
    /// `ROUNDS_TEST_PROFILE=<dir> cargo test rounds -- --ignored --nocapture`
    #[tokio::test]
    #[ignore]
    async fn updates_a_thunderstore_profile() {
        let dir = PathBuf::from(std::env::var("ROUNDS_TEST_PROFILE").expect("ROUNDS_TEST_PROFILE"));
        let http = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let plugins = dir.join("BepInEx/plugins");
        let sha = |path: &str| sha256(&fs::read(plugins.join(path)).unwrap());

        let first = update_profile(&dir, &http).await.unwrap();
        println!("first run:\n  {}", first.join("\n  "));

        // Bknibb's files, UnboundLib with the modpack's patch on top
        assert_eq!(sha("willis81808-UnboundLib/UnboundLib.dll"), "e817c732f769d562e46dd061f43212ed517f617b018a1af23aa68bfd032bf280");
        assert_eq!(sha("willis81808-UnboundLib/Octokit.dll"), OCTOKIT.sha256);
        assert_eq!(sha("willis81808-MMHook/MMHOOK_Assembly-CSharp.dll"), MMHOOK.sha256);
        assert_eq!(sha("olavim-RoundsWithFriends/RoundsWithFriends.dll"), RWF.sha256);
        assert!(plugins.join("OdinSerializer/Sirenix.Serialization.dll").is_file());
        assert!(dir.join("BepInEx/patchers/RoundsPort-AutoFix/rounds-port.AutoFix.dll").is_file());
        assert!(plugins.join("RoundsMacModpack-MacCompatFixes/MacCompatFixes.dll").is_file());

        // every modpack patch whose original is in the profile was applied
        for patch in patches() {
            for path in plugin_files(&plugins) {
                if util::fs::file_name_owned(&path) == patch.name {
                    assert_ne!(sha256(&fs::read(&path).unwrap()), patch.before, "{} not patched", path.display());
                }
            }
        }

        let cfg = fs::read_to_string(dir.join("BepInEx/config/BepInEx.cfg")).unwrap();
        assert!(cfg.contains("HideManagerGameObject = true"));

        let second = update_profile(&dir, &http).await.unwrap();
        assert!(second.is_empty(), "second run changed: {second:?}");
    }
}
