<p align="center">
  <a href="https://github.com/crystalux-project">
    <img src="https://github.com/crystalux-project/branding/blob/main/crystalux-logo-minimal.png?" alt="Logo" width="150" height="150">
  </a>
</p>
<p align="center"> 
<h2 align="center"> Amethyst</h2>
</p>
<p align="center">
<img src=https://img.shields.io/github/stars/crystalux-project/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/forks/crystalux-project/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/issues/crystalux-project/ame?style=flat&color=a900ff />
<img src=https://img.shields.io/github/issues-pr/crystalux-project/ame?style=flat&color=a900ff />
<a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW> </p></a>

<p align="center"> Amethyst is a fast, efficient and lightweight aur helper and pacman wrapper. 
Made for crystalux, compatible with any arch-based linux distribution.</p>

![](screenshot.png)

## Basic usage - <b>NOT COMPLETE!</b>
| Action | Command | Shorthand alias | Supported pacman equivalent |
| ------ | ------ | ------ | ------ |
| Install a package | amethyst install | ame ins | -S |
| Install a package via flatpak | amethyst flat | ame flat | -f |
| Remove a package| amethyst remove | ame rm | -R |
| Update repository | amethyst update | ame upd | -Sy |
| Upgrade a package | amethyst upgrade | ame upg | -Syu |
| Search for a package in repo and aur | amethyst search | ame sear | -Ss |
| Search for a package in repo | amethyst search -R | ame sear -R | -Ss -R |
| Search for a package in aur | amethyst search -A | ame sear -A | -Ss -A |

## How to build:
(Install cargo)

For release:
  - `make clean release`
 
For general debug/test:
  - `make clean debug`

Clean all build directories:
  - `make clean`
