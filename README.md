# Crosswind

[![GitHub License](https://img.shields.io/github/license/KieranK07/crosswind?style=flat)](LICENSE.md)

A mod manager for Thunderstore games, made for the Mac and on Windows too. On a Mac it lists every Thunderstore game
whose Steam version runs on a Mac and launches it with a Mac build of BepInEx (Apple Silicon and Intel). On Windows it
lists every Thunderstore game.

## Install

**Mac**

1. Download **Crosswind-macOS.zip** from [Releases](../../releases/latest), unzip it and drag **Crosswind** into Applications.
2. Open Crosswind. The first time, macOS blocks it because it isn't from the App Store: go to **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.

**Windows**

1. Download **Crosswind-Windows-setup.exe** from [Releases](../../releases/latest) and run it. If Windows says it protected your PC, click **More info → Run anyway**.
2. It installs next to Gale or r2modman and doesn't touch them.

Then choose a game, make a profile, install mods and, with Steam open, press **Launch**.

## What it does

- **Games:** on a Mac, the ones whose Steam page lists macOS, checked again each time Crosswind opens, so new
  Thunderstore games show up by themselves. On Windows, all of them.
- **Launching on a Mac:** the game starts directly with BepInEx loaded, not through Steam (which can't pass BepInEx's
  loader to a Mac game). Games on other mod loaders may not run on a Mac, and a mod that only works on Windows won't.
- **Profiles:** mods and their dependencies from Thunderstore, a config editor, and profiles shared as a code
  (**Export → profile as code**, then **Import → profile from code**), so Mac and Windows players can share one.
  Profiles from r2modman can be imported.

Problems: [open an issue](../../issues) with the game, the mod and the log (**File → Open profile folder**, then
`BepInEx/LogOutput.log`).

## Building

```
pnpm install
pnpm tauri build --target universal-apple-darwin --bundles app    # Mac
pnpm tauri build --bundles nsis                                    # Windows
```

The Mac's game list shipped with the app (used until the first check) comes from
`cd src-tauri && cargo test --lib write_bundled_games -- --ignored`.

## Credits

A fork of [Gale](https://github.com/Kesomannen/gale) by Kesomannen, so it's not Gale: please don't ask Gale's developer
for help with it. GPL-3.0, like Gale. Material icons are Apache 2.0. Thanks to Ebkr for r2modman.
