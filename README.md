[![Flathub Installs](https://img.shields.io/flathub/downloads/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/io.github.cosmic_utils.enroll)
[![Flathub Version](https://img.shields.io/flathub/v/org.cosmic_utils.enroll?logo=flathub)](https://flathub.org/apps/io.github.cosmic_utils.enroll)
# COSMIC™ Utils Enroll

GUI application for fingerprint management. Designed for COSMIC DE.


## Prerequisites

You're using Linux or freedesktop compatible system with a supported fingerprint scanner. You also need [fprintd][fprintd] running.

Tested with: 
| Distribution | Desktop | Architecture | Fingerprint scanner |
| ------------ | ------- | ------------ | ------------------- |
| Pop!_OS | COSMIC™ DE | amd64 | Goodix MOC Fingerprint Sensor | 
| Arch Linux | KDE Plasma | arm64 | No fingerprint scanner | 

## Usage

On a multiuser system you can choose user from navigation. It asks for authentication and checks correct rights if you choose user other than the user of current session.

Select the finger and action to take. Authentication and user rights check are performed for security. If anything goes wrong the status is displayed in the center. When registering a progress bar reflecting progress is shown. Follow instructions.

If you don't have correct rights or incorrect password your attempt is just dismissed.

## Installation

### Flathub
<a href='https://flathub.org/apps/io.github.cosmic_utils.enroll'><img width='240' alt='Get it on Flathub' src='https://flathub.org/api/badge?svg&locale=en'/></a>

### Build from source

[justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system

[just]: https://github.com/
[fprintd]: https://gitlab.freedesktop.org/libfprint/fprintd
