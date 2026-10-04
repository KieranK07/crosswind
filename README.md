# Crosswind

[![ROUNDS: tested](https://img.shields.io/badge/ROUNDS-tested-2ea44f?style=flat)](#what-it-does)
[![Other games: untested](https://img.shields.io/badge/other_games-untested-lightgrey?style=flat)](#crosswind)
[![GitHub License](https://img.shields.io/github/license/KieranK07/crosswind?style=flat)](LICENSE)

A Thunderstore mod manager for Mac and Windows, made for ROUNDS mods. It's a fork of [Gale](https://github.com/Kesomannen/gale) by Kesomannen, not Gale itself, so please don't ask Gale's developer for help with it.

> [!NOTE]
> Only ROUNDS is tested so far. Other games are listed because Gale lists them, and may not work, especially on a Mac.

## ROUNDS mods on a Mac (and Windows)

No Windows emulator, and no `old-rounds-for-mods` beta: that beta has no Mac version, which is why Steam keeps putting a Mac back on the newest ROUNDS. Crosswind makes ROUNDS mods built for the old game work on the current version instead.

### On a Mac (Apple Silicon or Intel)

1. Download **Crosswind-macOS.zip** from [Releases](../../releases/latest), unzip it and drag **Crosswind** into Applications.
2. Open Crosswind. The first time, macOS blocks it because it isn't from the App Store: go to **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.
3. Choose **ROUNDS**, make a profile and install mods as usual (UnboundLib, RoundsWithFriends, card packs...).
4. With Steam open, press **Launch**.

### Playing with friends on Windows

Everyone in a lobby needs the same version of ROUNDS and the same mods, and a Mac can't run the old beta. So your friends play on the current version too:

1. Steam → right-click ROUNDS → **Properties → Betas → None** (leave `old-rounds-for-mods`).
2. Download **Crosswind-Windows-setup.exe** from [Releases](../../releases/latest) and run it. If Windows says it protected your PC, click **More info → Run anyway**. It installs next to Gale or r2modman and doesn't touch them.
3. Share your mods: in Crosswind, **Export → ...profile as code**, and they use **Import → ...profile from code**.

### What it does

Before ROUNDS starts, Crosswind puts in [Bknibb](https://github.com/Bknibb)'s updated UnboundLib and RoundsWithFriends in place of the old ones, and fixes the other mods for the current game with [DuctTape](https://github.com/KieranK07/DuctTape). If you install an updated UnboundLib or RoundsWithFriends yourself, Crosswind uses yours. The 98 most-downloaded mods were played in real matches: see the [compatibility list](https://github.com/KieranK07/rounds-porting-toolkit/blob/main/docs/COMPATIBILITY.md). A mod still broken? Open an issue with its name and the log (**File → Open profile folder**, then `BepInEx/LogOutput.log`).

Building it yourself: `pnpm install && pnpm tauri build --bundles app` (Mac) or `--bundles nsis` (Windows). After DuctTape or the [toolkit](https://github.com/KieranK07/rounds-porting-toolkit) changes, `python scripts/rounds-sync.py` copies their files into `src-tauri/resources/rounds` first.

Everything below is Gale's own README.

## Features

- Support for 150+ games on Thunderstore and Hexium, including Lethal Company, R.E.P.O and Risk Of Rain 2
- An intuitive and responsive interface
- Tiny download size and resource usage
- Feature-rich mod config editor
- Automatic profile syncing

[...and more](https://github.com/Kesomannen/gale/wiki/Features)

## Installation

> [!WARNING]
> The only official sources for Gale are [Github](https://github.com/Kesomannen/gale), [Thunderstore](https://thunderstore.io/c/lethal-company/p/Kesomannen/GaleModManager/) and [Hexium](https://hexium.gg/mod-manager). Any other website claiming to provide an official download is not legit. Particularly, **galemodmanager.com** is unofficial and provides a severly outdated version of the app!

### Windows

<details>
  <summary>
    <b>Manual (Thunderstore)</b>
  </summary>
  
  - Go to the [Thunderstore page](https://thunderstore.io/c/lethal-company/p/Kesomannen/GaleModManager/) and click _Manual Download_.
  - Extract the downloaded .zip file (for example by right-clicking and choosing _Extract All_).
  - Run the `Gale_X.X.X_x64_en-US.msi` file inside of the extracted folder.
</details>

<details>
  <summary>
    <b>Manual (Github)</b>
  </summary>
  
  - Go to [Releases](https://github.com/Kesomannen/gale/releases).
  - Download the `Gale_X.X.X_x64_en-US.msi` file for your desired version (the latest is recommended).
  - Run the downloaded file.
</details>

<details>
  <summary>
    <b>Scoop</b>
  </summary>
  
  Gale is available as a [Scoop](https://scoop.sh/) app in the official [games bucket](https://github.com/Calinou/scoop-games):

```powershell
scoop bucket add games
scoop install gale
```

</details>

<details>
  <summary>
    <b>WinGet</b>
  </summary>
  
  Gale is available as a [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/) application:

```powershell
winget install Kesomannen.Gale
```

</details>

> [!NOTE]
> You might get a prompt saying "Windows has protected your PC". In this case, click `More Info` and `Run Anyway`.

> [!TIP]
> If you're unsure about the safety of this app, I would suggest running it through a service like [VirusTotal](https://www.virustotal.com).

### Linux

<details>
  <summary>
    <b>Arch</b>
  </summary>
  
  Gale is available as a **community-maintained** AUR package: [gale](https://aur.archlinux.org/packages/gale) (from source) and [gale-bin](https://aur.archlinux.org/packages/gale-bin) (prebuilt).
  
  Example installation command:
  
  ```bash
  yay -S gale-bin
  ```

> [!WARN]
> **Do not** use the in-app updater, instead update the app via the AUR.

</details>

<details>
  <summary>
    <b>Debian</b>
  </summary>

Gale is available as a .deb package in [Releases](https://github.com/Kesomannen/gale/releases). After downloading, install with:

```bash
sudo dpkg -i Gale_X.X.X_x64_en-US.deb
```

Updating Gale can be done from the in-app updater UI.

</details>

<details>
  <summary>
    <b>Fedora</b>
  </summary>

Gale is available as a .rpm package in [Releases](https://github.com/Kesomannen/gale/releases). After downloading, install with:

```bash
sudo rpm -i Gale_X.X.X_x64_en-US.rpm
```

Updating Gale can be done from the in-app updater UI.

</details>

<details>
  <summary>
    <b>Flatpak</b>
  </summary>

Gale is available as an independently hosted Flatpak package:

```bash
flatpak install https://kesomannen.com/com.kesomannen.gale.flatpakref
```

Updating the app can be done with `flatpak update com.kesomannen.gale`.

</details>

<details>
  <summary>
    <b>AppImage</b>
  </summary>

Distribution-agnostic AppImages are available in [Releases](https://github.com/Kesomannen/gale/releases). After downloading, make the file executable and run it:

```bash
chmod +x Gale_X.X.X_x64_en-US.AppImage
./Gale_X.X.X_x64_en-US.AppImage
```

Updating Gale can be done from the in-app updater UI.

</details>

---

Want to build it from source? See the [wiki](https://github.com/Kesomannen/gale/wiki/building-from-source).

## Screenshots

_Profile_

![screenshot](https://raw.githubusercontent.com/Kesomannen/gale/master/images/screenshots/screenshot1.png)

_Thunderstore browser_

![screenshot](https://raw.githubusercontent.com/Kesomannen/gale/master/images/screenshots/screenshot2.png)

_Mod config editor_

![screenshot](https://raw.githubusercontent.com/Kesomannen/gale/master/images/screenshots/screenshot3.png)

_Modpack export_

![screenshot](https://raw.githubusercontent.com/Kesomannen/gale/master/images/screenshots/screenshot4.png)

## Credits

Material icons licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.html).

Thanks to Ebkr for helping to navigate the Thunderstore API and BepInEx, and of course making the original mod manager!

---

Still have questions? See the [FAQ](https://github.com/Kesomannen/gale/wiki/faq) or a [detailed list of features](https://github.com/Kesomannen/gale/wiki/Features).
