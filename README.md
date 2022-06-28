<p align="center">
  <a href="https://github.com/crystal-linux/amethyst/">
    <img src="https://getcryst.al/site/assets/other/logo.png" alt="Logo" width="150" height="150">
  </a>
</p>

<h2 align="center"> Amethyst</h2>

<p align="center">
    <a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW> </a>
    <a href="https://github.com/crystal-linux/amethyst"><img src="https://github.com/crystal-linux/amethyst/actions/workflows/test.yml/badge.svg"></a>
    <img src="https://img.shields.io/badge/Maintainer-@ihatethefrench-brightgreen" alt=The maintainer of this repository" href="https://github.com/ihatethefrench">
</p>

<p align="center"> 
Amethyst is a fast, efficient and lightweight AUR helper and Pacman wrapper.<br> 
Made for Crystal, compatible with any Arch-based Linux distribution.
</p>

### Basic usage

| Action               | FreeBSD pkg-style alias | Pacman-style flags |
|----------------------|-------------------------|--------------------|
| Install a package    | ame ins/install         | ame -S             |
| Remove a package     | ame rm/remove           | ame -R/-Rs         |
| Upgrade a package    | ame upg/upgrade         | ame -Syu           |
| Search for a package | ame sea                 | ame -Ss            |

### Exit codes overview

| Exit Code (i32) | Reason                                                   |
|-----------------|----------------------------------------------------------|
| 1               | Running ame as UID 0 / root                              |
| 2               | Failed adding package to database                        |
| 3               | Failed initialising database                             |
| 4               | Error creating cache and/or database paths               |
| 5               | Could not find one or more required package dependencies |
| 6               | User cancelled package installation                      |
| 7               | Pacman error when installing package                     |
| 8               | Git error                                                |
| 9               | makepkg error                                            |

### How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`

#### Pkg-warner included

- `cargo build (--release) --all --features=pkg-warner`

<!--

echo "AME_UWU=true" >> ~/.zshrc
echo "AME_UWU=true" >> ~/.bashrc
set -Ux AME_UWU true

:)

-->
