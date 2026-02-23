# COSMIC™ Utils Enroll

GUI for fingerprint management. Designed for COSMIC DE.

## Prerequisites

You're using Linux or freedesktop compatible system with a supported fingerprint scanner. Tested with COSMIC™ DE, Pop!_OS, Framework 13 laptop with a Goodix MOC Fingerprint Sensor. 

## Usage

Choose which finger to register or delete by a tab. Change user from a menu (default is current session.)</br> Click the action you want to take. Prompts you for your password. Follow instruction.</br> If you don't have correct rights or incorrect password your attempt will be dismissed.

[recording-2026-02-16_00-19-25.webm](https://github.com/user-attachments/assets/5c22b844-157a-41f2-9c07-83a073bd0d6b)

## Todos

- [x] Improve feedback given to user. Currently prints what daemon returns.
- [x] Add a user dropdown to make it possible for admin to register for other users.
- [x] Get project into cosmic-utils.
- [x] Package & distribute, most likely as a flatpak, but maybe as a deb package also.
- [ ] Sherlock the application by adding all functionality directly into cosmic-settings.


## Installation

Download the .flatpak from latest release or build;


[justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
