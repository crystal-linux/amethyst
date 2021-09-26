<p align="center">
  <a href="https://github.com/crystal-linux">
    <img src="https://raw.githubusercontent.com/crystal-linux/branding/main/logos/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>
<p align="center"> 
<h2 align="center"> Amethyst</h2>
</p>
<p align="center">
<img src=https://img.shields.io/github/stars/crystal-linux/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/forks/crystal-linux/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/issues/crystal-linux/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/issues-pr/crystal-linux/ame?style=flat&color=a900ff />
<a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW> </p></a>

<p align="center"> Amethyst is a fast, efficient and lightweight aur helper and pacman wrapper. 
Made for crystalux, compatible with any arch-based linux distribution.</p>

![](screenshot.png)

## Basic usage
| Action | FreeBSD pkg-style alias | Pacman-style flag(s) |
| ------ | ------ | ------ |
| Install a package | ame ins | ame -S |
| Remove a package | ame rm | ame -R / -Rs |
| Update repository | ame upd | ame -Sy |
| Upgrade a package | ame upg | ame -Syu |
| Search for a package in general | ame sea | ame -Ss |
| Search for a package in the repos | ame repsea | ame -Sr |
| Search for a package in aur | ame aursea | ame -S |

## How to build:
(Install cargo)

For release:
  - `make clean release`
 
For general debug/test:
  - `make clean debug`

Clean all build directories:
  - `make clean`
