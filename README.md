<p align="center">
  <a href="https://github.com/crystal-linux/amethyst/">
    <img src="https://getcryst.al/assets/img/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>

<h2 align="center"> Amethyst</h2>

<p align="center">
    <a href="https://github.com/crystal-linux/amethyst"><img src="https://github.com/crystal-linux/amethyst/actions/workflows/test.yml/badge.svg"></a>
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License">
    <a href="https://github/crystal-linux/amethyst"><img alt="GitHub isses" src="https://img.shields.io/github/issues-raw/crystal-linux/amethyst"></a>
    <a href="https://github/crystal-linux/amethyst"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/amethyst"></a><br>
    <a href="https://twitter.com/intent/user?screen_name=crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux?style=flat?color=blue">
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
   <a href="https://github.com/ihatethefrench"> <img src="https://img.shields.io/badge/Maintainer-@ihatethefrench-brightgreen" alt=The maintainer of this repository" href="https://github.com/ihatethefrench"></a>
 </a>
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

### TODO:

#### v3.3.0
- ~~Add clean function~~
- ~~Make flags global~~
- ~~Add pacdiff function~~
- ~~Squash any remaining bugs~~

#### v3.4.0
- ~~New AUR upgrade checker~~
- ~~Bugfixes, bugfixes, bugfixes!~~~

#### v3.5.0 
- Implement some sort of spinner for longer operations
- Allow editing of PKGBUILDs before install
- Allow to choose provider of package
- Highlight optdepends at the end of an install operation

<!--

echo "AME_UWU=true" >> ~/.zshrc
echo "AME_UWU=true" >> ~/.bashrc
set -Ux AME_UWU true

:)

-->
