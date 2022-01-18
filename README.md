<p align="center">
  <a href="https://git.getcryst.al/crystal/ame/">
    <img src="https://git.getcryst.al/crystal/branding/raw/branch/main/logos/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>
<p align="center"> 
<h2 align="center"> Amethyst</h2>
</p>
<p align="center">
<a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW> </p></a>

<p align="center"> Amethyst is a fast, efficient and lightweight AUR helper and Pacman wrapper. 
Made for Crystal, compatible with any Arch-based Linux distribution.</p>

![](screenshot.png)

## Basic usage
| Action               | FreeBSD pkg-style alias | Pacman-style flag(s) |
|----------------------|-------------------------|----------------------|
| Install a package    | ame ins/install         | ame -S               |
| Remove a package     | ame rm/remove           | ame -R/-Rs           |
| Upgrade a package    | ame upg/upgrade         | ame -Syu             |
| Search for a package | ame sea                 | ame -Ss              |


## How to build:
(Install cargo)

For release:
  - `make clean release`
 
For general debug/test:
  - `make debug`

Clean all build directories:
  - `make clean`

<br>
<br>

```sh
echo "AME_UWU=YES" >> ~/.zshrc # for zsh
echo "AME_UWU=YES" >> ~/.bashrc # for bash
set -Ux AME_UWU YES # for fish
```