## Pull requests

Expensive CI is run against every commit to main. That is why I ask you to open pull requests against the versioned branch that is most appropriate. Which at the time of writing is the only branch with version number.

However if you are committing just translations and do those through GitHub or any other reason found yourself opening PR against main then no worries. I will edit it to target the correct release. 

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Todos

- [x] Improve feedback given to user. Currently prints what daemon returns.
- [x] Add a user dropdown to make it possible for admin to register for other users.
- [x] Get project into cosmic-utils.
- [x] Package & distribute, most likely as a flatpak, but maybe as a deb package also.
- [ ] Adding Help page into View
- [ ] Sherlock the application by adding all functionality directly into cosmic-settings.

## Packaging

### Apt

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/cosmic-utils-enroll prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment. .deb package to [Debian][debian] would be a good start.

### Flatpak

Every build generates a .flatpak artifact. Submitting to [Flathub][flathub] next.

[fluent]: https://projectfluent.org/
[iso-codes]: https://en.wikipedia.org/wiki/ISO_639-1
[fluent-guide]: https://projectfluent.org/fluent/guide/
[flathub]: https://flathub.org/
[debian]: https://www.debian.org/
