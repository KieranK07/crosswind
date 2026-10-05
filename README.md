# Crosswind

[![GitHub License](https://img.shields.io/github/license/KieranK07/crosswind?style=flat)](LICENSE.md)

A mod manager for Thunderstore games on a Mac. It lists every Thunderstore game whose Steam version runs on a Mac,
installs mods into profiles, and launches the game with a Mac build of BepInEx. Apple Silicon and Intel.

## Install

1. Download **Crosswind-macOS.zip** from [Releases](../../releases/latest), unzip it and drag **Crosswind** into Applications.
2. Open Crosswind. The first time, macOS blocks it because it isn't from the App Store: go to **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.
3. Choose a game, make a profile and install mods.
4. With Steam open, press **Launch**.

## What it does

- **Games:** the ones whose Steam page lists macOS. The list is checked again each time Crosswind opens, so new
  Thunderstore games show up by themselves.
- **Launching:** the game starts directly with BepInEx loaded, not through Steam (which can't pass BepInEx's loader to
  a Mac game). Games on other mod loaders may not run on a Mac.
- **Profiles:** mods and their dependencies from Thunderstore, a config editor, and profiles shared as a code
  (**Export → profile as code**, then **Import → profile from code**). Profiles from r2modman can be imported.

A mod that only works on Windows still won't run. Problems: [open an issue](../../issues) with the game, the mod and
the log (**File → Open profile folder**, then `BepInEx/LogOutput.log`).

## Building

```
pnpm install
pnpm tauri build --target universal-apple-darwin --bundles app
```

The game list shipped with the app (used until the first check) comes from
`cd src-tauri && cargo test --lib write_bundled_games -- --ignored`.

## Credits

A fork of [Gale](https://github.com/Kesomannen/gale) by Kesomannen, so it's not Gale: please don't ask Gale's developer
for help with it. GPL-3.0, like Gale. Material icons are Apache 2.0. Thanks to Ebkr for r2modman.
