[![Flathub Installs](https://img.shields.io/flathub/downloads/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll)
[![Flathub Version](https://img.shields.io/flathub/v/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll)
[![AUR Version](https://img.shields.io/aur/version/enroll)](https://aur.archlinux.org/packages/enroll)
<a href="https://hosted.weblate.org/engage/cosmic-utils/"><img src="https://hosted.weblate.org/widget/cosmic-utils/enroll/svg-badge.svg" alt="Translation status"></a>
# Enroll

Application for fingerprint management. Designed for COSMIC DE.

## Prerequisites

Linux with a supported fingerprint scanner. [Fprintd][fprintd] or [open-fprintd][open-fprintd] installed.

#### Tested with: 
| Distribution | Desktop | Architecture | Fingerprint scanner |
| ------------ | ------- | ------------:|:-------------------:|
| Pop!_OS | COSMIC™ DE | amd64 | Goodix MOC Fingerprint Sensor | 
| Arch Linux | KDE Plasma | arm64 | No fingerprint scanner | 

#### Usage

Choose which finger and press Register.

## Installation

#### Flathub
<a href='https://flathub.org/apps/org.github.cosmic_utils.enroll'><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?svg&locale=en'/></a>
#### AUR
You can just [go get it][aur] but I recommend installing using [Shelly][shelly] for instance.
#### AppImage
Download from [Releases][latest-release]

#### Fedora
Available at [Copr][copr]. Install:

```sh
sudo dnf copr install jwnz/Enroll
```

#### Ubuntu / Debian / Pop!_OS
Available at [Launchpad][launchpad]. Install:

```sh
sudo add-apt-repository ppa:jotuel/enroll
sudo apt update
sudo apt install cosmic-utils-enroll
```

[shelly]: https://www.seafoam-labs.org/shelly-alpm
[aur]: https://aur.archlinux.org/packages/enroll
[fprintd]: https://fprint.freedesktop.org/
[open-fprintd]: https://github.com/uunicorn/open-fprintd
[latest-release]: /../../releases/latest
[copr]: https://copr.fedorainfracloud.org/coprs/jwnz/Enroll/
[launchpad]: https://launchpad.net/~jotuel/+archive/ubuntu/enroll
