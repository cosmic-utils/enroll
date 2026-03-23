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

Select the finger and action to take. Authentication and rights check are performed so that. If something goes wrong the status is text in the center. When registering a progress bar reflecting progress is shown. Do as instructed.

If you don't have correct rights or incorrect password your attempt is just dismissed.

## Installation

### Flathub
Get from [here][flathub].

### Build from source

[justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system

[flathub]: https://flathub.org/en/apps/org.cosmic_utils.enroll
[just]: https://github.com/
[fprintd]: https://gitlab.freedesktop.org/libfprint/fprintd
