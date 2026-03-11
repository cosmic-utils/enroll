# COSMIC™ Utils Enroll

GUI application for fingerprint management. Designed for COSMIC DE.

## Prerequisites

You're using Linux or freedesktop compatible system with a supported fingerprint scanner. 

Tested only with COSMIC™ DE, Pop!_OS, Framework 13 laptop with a Goodix MOC Fingerprint Sensor. 

## Usage

On a multiuser system you can choose user from navigation. It asks for authentication and checks correct rights if you choose user other than current session.

Click the action you want to take. Different kind of authentication and rights check is performed. If something goes wrong status is shown. Otherwise you'll get a progress indicator. If you don't have correct rights or incorrect password your attempt will be dismissed.

## Installation

Download the .flatpak from latest release or build;


[justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
