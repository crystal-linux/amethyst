<p align="center">
  <a href="https://github.com/crystal-linux/amethyst/">
    <img src="./logo.svg" alt="Logo" width="150" height="150">
  </a>
</p>

<h2 align="center">Amethyst</h2>

<p align="center">
    <a href="https://getcryst.al/site/docs/amethyst/getting-started"><img src="https://img.shields.io/badge/Documentation-Click%20here!-informational"></a>
    <a href="https://github.com/crystal-linux/amethyst"><img src="https://github.com/crystal-linux/amethyst/actions/workflows/test.yml/badge.svg"></a><br>
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License"></a>
    <a href="https://github.com/crystal-linux/amethyst/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues-raw/crystal-linux/amethyst"></a>
    <a href="https://github.com/crystal-linux/amethyst/pulls"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/amethyst"></a><br>
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
    <a href="https://github.com/not-my-segfault"><img src="https://img.shields.io/badge/Maintainer-@not%2D-my%2D-segfault-brightgreen" alt="The maintainer of this repository" href="https://github.com/not-my-segfault"></a><br>
    <a href="https://fosstodon.org/@crystal_linux"><img alt="Mastodon Follow" src="https://img.shields.io/mastodon/follow/108618426259408142?domain=https%3A%2F%2Ffosstodon.org"></a>
    <a href="https://twitter.com/crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux"></a>
</p>

<p align="center"> 
Amethyst is a fast, efficient and lightweight AUR helper and Pacman wrapper.<br> 
Made for Crystal, compatible with any Arch-based Linux distribution.
</p>

### Basic usage

| Action                 | FreeBSD pkg-style alias | Pacman-style flags |
|------------------------|-------------------------|--------------------|
| Install a package      | ame ins/install         | ame -S             |
| Remove a package       | ame rm/remove           | ame -R/-Rs         |
| Upgrade a package      | ame upg/upgrade         | ame -Syu           |
| Search for a package   | ame sea/search          | ame -Ss            |
| Query the package list | ame qu/query            | ame -Q             |    
| Show a package's info  | ame inf/info            | ame -Qi            |
| Clean the pacman cache | ame cl/clean            | ame -Sc            |
| Check for .pacnew      | ame di/diff             | ame -D             |

### Exit codes overview

| Exit Code (i32) | Reason                                                   |
|-----------------|----------------------------------------------------------|
| 1               | Running ame as UID 0 / root                              |
| 2               | Failed creating paths                                    |
| 3               | Could not find one or more required package dependencies |
| 4               | User cancelled package installation                      |
| 5               | Pacman error when installing package                     |
| 6               | Git error                                                |
| 7               | Makepkg error                                            |
| 8               | Failed to parse config file                              |
| 63              | Any other misc error                                     |

### Install on non-Crystal distros
- `sudo pacman -S --needed base-devel pacman-contrib cargo`
- `git clone https://github.com/crystal-linux-packages/ame`
- `cd ame && makepkg -si`

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`

### TODO:

#### v3.6.0
- ~~Allow editing of PKGBUILDs before install~~

<!--

echo "AME_UWU=true" >> ~/.zshrc
echo "AME_UWU=true" >> ~/.bashrc
set -Ux AME_UWU true

:)

-->
