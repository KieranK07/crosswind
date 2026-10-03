//! Launching native macOS games with BepInEx.
//!
//! Thunderstore's BepInEx packs are Windows builds (a `winhttp.dll` proxy), and Steam can't pass
//! environment variables to a game it starts. So on macOS the game's executable is started directly,
//! with Unity Doorstop injected through `DYLD_INSERT_LIBRARIES`. Doorstop and a BepInEx core that
//! runs natively on Apple Silicon (BepInEx v5-lts, unreleased past 5.4.23.5) ship with the app and
//! are written into the profile before each modded launch.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use eyre::{Context, OptionExt, Result};
use include_dir::{Dir, include_dir};
use tracing::info;

use crate::game::Game;

static MAC_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources/macos");

const DOORSTOP_LIB: &str = "libdoorstop.dylib";

/// Writes Doorstop and the macOS BepInEx core into the profile, replacing the files of the
/// Windows pack. Files that already match are left alone.
pub fn prepare_profile(profile_dir: &Path) -> Result<()> {
    let mut written = 0;
    write_dir(&MAC_FILES, profile_dir, &mut written)?;

    if written > 0 {
        info!("wrote {written} macOS BepInEx files into the profile");
    }

    Ok(())
}

fn write_dir(dir: &Dir, profile_dir: &Path, written: &mut usize) -> Result<()> {
    for file in dir.files() {
        let path = profile_dir.join(file.path());

        if fs::read(&path).is_ok_and(|current| current == file.contents()) {
            continue;
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Profile files are often hard links into Gale's cache: replace the link, never write
        // through it.
        if path.exists() {
            fs::remove_file(&path).with_context(|| format!("failed to remove {}", path.display()))?;
        }

        fs::write(&path, file.contents())
            .with_context(|| format!("failed to write {}", path.display()))?;
        *written += 1;
    }

    for sub in dir.dirs() {
        write_dir(sub, profile_dir, written)?;
    }

    Ok(())
}

/// The command that starts the game's app bundle directly, outside Steam. Steam must be running.
pub fn game_command(game_dir: &Path, game: Game) -> Result<Command> {
    let exe = find_app_executable(game_dir)?;

    let mut command = Command::new(&exe);
    command.current_dir(game_dir);

    if let Some(steam) = &game.platforms.steam {
        // A game started outside Steam can't reach Steam unless it's already running.
        let running = Command::new("pgrep")
            .args(["-x", "steam_osx"])
            .output()
            .is_ok_and(|output| output.status.success());
        if !running {
            let _ = Command::new("open").args(["-g", "-a", "Steam"]).spawn();
            eyre::bail!("Steam wasn't running, so it's starting now. Launch again once Steam is open.");
        }

        // Without these the Steam API doesn't know which game it is and the game restarts
        // itself through Steam, dropping Doorstop.
        let id = steam.id.to_string();
        command.env("SteamAppId", &id).env("SteamGameId", &id);

        let appid_file = game_dir.join("steam_appid.txt");
        if !appid_file.exists()
            && let Err(err) = fs::write(&appid_file, &id)
        {
            tracing::warn!("failed to write {}: {err:#}", appid_file.display());
        }
    }

    Ok(command)
}

/// Sets the environment Doorstop reads (the same variables its `run_bepinex.sh` exports).
pub fn add_doorstop_env(command: &mut Command, profile_dir: &Path) -> Result<()> {
    let doorstop = profile_dir.join(DOORSTOP_LIB);
    let preloader = profile_dir.join("BepInEx/core/BepInEx.Preloader.dll");

    let inserted = match std::env::var("DYLD_INSERT_LIBRARIES") {
        Ok(existing) if !existing.is_empty() => format!("{}:{existing}", doorstop.display()),
        _ => doorstop.display().to_string(),
    };

    command
        .env("DYLD_INSERT_LIBRARIES", inserted)
        .env("DOORSTOP_ENABLED", "1")
        .env("DOORSTOP_TARGET_ASSEMBLY", preloader)
        .env("DOORSTOP_IGNORE_DISABLED_ENV", "0")
        .env("DOORSTOP_BOOT_CONFIG_OVERRIDE", "")
        .env("DOORSTOP_MONO_DLL_SEARCH_PATH_OVERRIDE", "")
        .env("DOORSTOP_MONO_DEBUG_ENABLED", "0")
        .env("DOORSTOP_MONO_DEBUG_ADDRESS", "127.0.0.1:10000")
        .env("DOORSTOP_MONO_DEBUG_SUSPEND", "0");

    Ok(())
}

/// `<game_dir>/<Name>.app/Contents/MacOS/<executable>`.
fn find_app_executable(game_dir: &Path) -> Result<PathBuf> {
    let app = game_dir
        .read_dir()
        .with_context(|| format!("failed to read {}", game_dir.display()))?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "app"))
        .min()
        .ok_or_eyre("no .app bundle found in the game directory")?;

    let macos_dir = app.join("Contents/MacOS");

    // CFBundleExecutable is almost always the bundle's name; otherwise take the only file there.
    let by_name = app
        .file_stem()
        .map(|stem| macos_dir.join(stem))
        .filter(|path| path.is_file());

    by_name
        .or_else(|| {
            macos_dir
                .read_dir()
                .ok()?
                .filter_map(std::result::Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_file())
                .min()
        })
        .ok_or_else(|| eyre::eyre!("no executable found in {}", macos_dir.display()))
}
