[![Flathub Installs](https://img.shields.io/flathub/downloads/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll)
[![Flathub Version](https://img.shields.io/flathub/v/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll)
[![AUR Version](https://img.shields.io/aur/version/enroll)](https://aur.archlinux.org/packages/enroll)
<a href="https://hosted.weblate.org/engage/cosmic-utils/"><img src="https://hosted.weblate.org/widget/cosmic-utils/enroll/svg-badge.svg" alt="Translation status"></a>
# Enroll

GUI application for fingerprint management. Designed for COSMIC DE.

## Prerequisites

Linux with a supported fingerprint scanner. You also need [fprintd][fprintd] or [open-fprintd][open-fprintd] installed.

### Tested with: 
| Distribution | Desktop | Architecture | Fingerprint scanner |
| ------------ | ------- | ------------:|:-------------------:|
| Pop!_OS | COSMIC™ DE | amd64 | Goodix MOC Fingerprint Sensor | 
| Arch Linux | KDE Plasma | arm64 | No fingerprint scanner | 

## Usage

On a multiuser system you can choose user from navigation. It asks for authentication and checks correct rights if you choose user other than the user of current session.

Select the finger and action to take. Authentication and user rights check are performed for security. If anything goes wrong the status is displayed in the center. When registering a progress bar reflecting progress is shown. Follow instructions.

If you don't have correct rights or incorrect password your attempt is just dismissed.

## Installation

### Flathub
<a href='https://flathub.org/apps/org.github.cosmic_utils.enroll'><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?svg&locale=en'/></a>
### AUR
You can just [go get it][aur] but I recommend installing using [Shelly][shelly] for instance.
### AppImage
Download from [Releases][latest-release]

[shelly]: https://www.seafoam-labs.org/shelly-alpm
[aur]: https://aur.archlinux.org/packages/enroll
[fprintd]: https://fprint.freedesktop.org/
[open-fprintd]: https://github.com/uunicorn/open-fprintd
[latest-release]: /../../releases/latest
