## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment.

### Launchpad

Adding debian folder with relevant files &

`just rootdir=debian/cosmic-utils-enroll prefix=/usr install`

### AUR

Creating a `PKGBUILD` and submitting there. I don't know whether the license is compatible.

### Copr

Any GitHub Action needed to automate can be added.

[fluent]: https://projectfluent.org/
[iso-codes]: https://en.wikipedia.org/wiki/ISO_639-1
[fluent-guide]: https://projectfluent.org/fluent/guide/
[flathub]: https://flathub.org/
[debian]: https://www.debian.org/
