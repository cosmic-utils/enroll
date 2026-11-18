# <img width="30" src="../resources/icons/hicolor/scalable/apps/enroll.svg"> Enroll ![GitHub Release](https://img.shields.io/github/v/release/cosmic-utils/enroll) <a href="https://hosted.weblate.org/engage/cosmic-utils/"><img src="https://hosted.weblate.org/widget/cosmic-utils/enroll/svg-badge.svg" alt="Translation status"></a>

Application for fingerprint management. Designed for COSMIC DE.

### Prerequisites

Linux with a supported fingerprint scanner. [Fprintd][fprintd] or [open-fprintd][open-fprintd] installed.

#### Usage

Choose a *finger* and press **Register**.

## Installing

#### Arch Linux [![AUR Version](https://img.shields.io/aur/version/enroll)](https://aur.archlinux.org/packages/enroll) ![AUR Maintainer](https://img.shields.io/aur/maintainer/enroll)

Available from [AUR][aur]. I recommend installing using [Shelly][shelly].
#### AppImage ![GitHub Release](https://img.shields.io/github/v/release/cosmic-utils/enroll)
Download from [Releases][latest-release]. Install using [Gear Lever][gearlever].

```sh
chmod +x cosmic-utils-enroll-x86_64.AppImage
./cosmic-utils-enroll-x86_64.AppImage
```

#### Fedora [![Copr build status](https://copr.fedorainfracloud.org/coprs/jwnz/Enroll/package/cosmic-utils-enroll/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/jwnz/Enroll/package/cosmic-utils-enroll/)
Available from [Copr][copr]. Install:

```sh
sudo dnf copr install jwnz/Enroll
sudo dnf install cosmic-utils-enroll
```
#### Flathub [![Flathub Version](https://img.shields.io/flathub/v/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll) [![Flathub Installs](https://img.shields.io/flathub/downloads/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/org.cosmic_utils.enroll) 
<a href='https://flathub.org/apps/org.github.cosmic_utils.enroll'><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?svg&locale=en'/></a>

#### Debian / Ubuntu / Pop!_OS <a href="https://launchpad.net/~jotuel/+archive/ubuntu/enroll" style="border: none;"><img src="https://ubuntu.com/docs/launchpad/_images/Launchpad_120px_badge.png" alt="Launchpad logo" style="border: none;"/></a> 

```sh
sudo add-apt-repository ppa:jotuel/enroll
sudo apt update
sudo apt install cosmic-utils-enroll
```
#### NixOS [![version](https://repology.org/badge/version-for-repo/nix_unstable/enroll.svg)](https://repology.org/project/enroll/versions)
Available from [Nix unstable][nix]:

```sh
  environment.systemPackages = [
    pkgs.enroll
  ];
```

[shelly]: https://www.seafoam-labs.org/shelly-alpm
[aur]: https://aur.archlinux.org/packages/enroll
[fprintd]: https://fprint.freedesktop.org/
[open-fprintd]: https://github.com/uunicorn/open-fprintd
[latest-release]: /../../releases/latest
[copr]: https://copr.fedorainfracloud.org/coprs/jwnz/Enroll/
[launchpad]: https://launchpad.net/~jotuel/+archive/ubuntu/enroll
[nix]: https://search.nixos.org/packages?channel=unstable&query=enroll#show=enroll
[gearlever]: https://github.com/mijorus/gearlever
