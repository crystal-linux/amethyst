<p align="center">
  <a href="https://github.com/crystal-linux">
    <img src="https://git.getcryst.al/crystal/branding/raw/branch/main/logos/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>
<p align="center"> 
<h2 align="center"> Amethyst</h2>
</p>
<p align="center">
<img src=https://img.shields.io/github/stars/crystal-linux/ame?style=flat&color=a900ff&logo=Github />
<img src=https://img.shields.io/github/forks/crystal-linux/ame?style=flat&color=a900ff&logo=Github />
<img src=https://img.shields.io/github/issues/crystal-linux/ame?style=flat&color=a900ff&logo=Github />
<img src=https://img.shields.io/github/issues-pr/crystal-linux/ame?style=flat&color=a900f&logo=Github />
<a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW> </p></a>

<p align="center"> Amethyst is a fast, efficient and lightweight AUR helper and Pacman wrapper. 
Made for Crystal, compatible with any Arch-based Linux distribution.</p>

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
| Search for a package in aur | ame aursea | ame -Sa |

## How to build:
(Install cargo)

For release:
  - `make clean release`
 
For general debug/test:
  - `make debug`

Clean all build directories:
  - `make clean`


`echo "AME_UWU=YES" >> ~/.zshrc`, self explanatory 