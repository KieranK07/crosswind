![Gale](https://raw.githubusercontent.com/Kesomannen/gale/master/images/banner.png)

[![Thunderstore Version](https://img.shields.io/thunderstore/v/Kesomannen/GaleModManager?style=flat)](https://thunderstore.io/c/lethal-company/p/Kesomannen/GaleModManager/)
[![Discord](https://img.shields.io/discord/1288196347597688912?style=flat&label=discord)](https://discord.gg/sfuWXRfeTt)
[![GitHub License](https://img.shields.io/github/license/Kesomannen/gale?style=flat)](https://github.com/Kesomannen/gale?tab=GPL-3.0-1-ov-file#readme)

A powerful mod manager for [Thunderstore](https://thunderstore.io), built with [Svelte](https://kit.svelte.dev/) and [Tauri](https://tauri.app/).

## ROUNDS mods on a Mac (and Windows)

This is Gale, the Thunderstore mod manager, with two changes: it runs on Mac, and it makes ROUNDS mods built for the old game work on the current version. No Windows emulator, and no `old-rounds-for-mods` beta: that beta has no Mac version, which is why Steam keeps putting a Mac back on the newest ROUNDS.

### On a Mac (M1 or newer)

1. Download **Gale-macOS.zip** from [Releases](../../releases/latest), unzip it and drag **Gale** into Applications.
2. Open Gale. The first time, macOS blocks it because it isn't from the App Store: go to **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.
3. Choose **ROUNDS**, make a profile and install mods as usual (UnboundLib, RoundsWithFriends, card packs...).
4. With Steam open, press **Launch**.

### Playing with friends on Windows

Everyone in a lobby needs the same version of ROUNDS and the same mods, and a Mac can't run the old beta. So your friends play on the current version too:

1. Steam → right-click ROUNDS → **Properties → Betas → None** (leave `old-rounds-for-mods`).
2. Install Gale from [Releases](../../releases/latest) (the `-setup.exe`). Already using Gale? Their profiles carry over.
3. Share your mods: in Gale, **Export → ...profile as code**, and they use **Import → ...profile from code**.

### What it does

Before ROUNDS starts, Gale puts in [Bknibb](https://github.com/Bknibb)'s updated UnboundLib and RoundsWithFriends in place of the old ones, and fixes the other mods for the current game ([ROUNDS Porting Toolkit](https://github.com/KieranK07/rounds-porting-toolkit)). If you install an updated UnboundLib or RoundsWithFriends yourself, Gale uses yours. The 98 most-downloaded mods were played in real matches: see the [compatibility list](https://github.com/KieranK07/rounds-porting-toolkit/blob/main/docs/COMPATIBILITY.md). A mod still broken? Open an issue with its name and Gale's log (**File → Open profile folder**, then `BepInEx/LogOutput.log`).

Building it yourself: `pnpm install && pnpm tauri build --bundles app` (Mac) or `--bundles nsis` (Windows). After the toolkit changes, `python scripts/rounds-sync.py` copies its files into `src-tauri/resources/rounds` first. Everything below is upstream Gale's README.

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
