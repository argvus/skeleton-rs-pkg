<!--
  Template repository for ARGVUS Arch Linux packages.

  This repository is a starting point, not a shipped project. When you create
  a new packaging project from this template, adapt it like this:

  - Replace every placeholder below: <OWNER>, <REPO>, <PKGNAME>, <PKGDESC>.
  - Fill in the package metadata in both PKGBUILDs: pkgname, pkgver, url and
    depends.
  - Replace the example crates with the real application crates.
  - Update .github/CODEOWNERS and the package metadata (pkgdesc, license).
  - Remove this comment block.
-->

# skeleton

Rust workspace and Arch Linux package skeleton for ARGVUS projects.

[![CI](https://github.com/argvus/skeleton-rs-pkg/actions/workflows/ci.yml/badge.svg)](https://github.com/argvus/skeleton-rs-pkg/actions/workflows/ci.yml)
[![Release](https://github.com/argvus/skeleton-rs-pkg/actions/workflows/release.yml/badge.svg)](https://github.com/argvus/skeleton-rs-pkg/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

The repository contains a small binary crate, a reusable core crate, local
package tooling, and a release workflow for publishing a signed Arch package.
Use it as a starting point: replace the package metadata, application logic,
repository URLs, and maintainer information before creating a derived project.

## Install

From the ARGVUS repository:

```sh
sudo pacman -S skeleton
```

## Build from source

```sh
make build
make install
```

`make build` runs the Rust checks and creates the local package at
`build/dist/`. `make rust-build` only compiles the workspace in debug mode.

## Workspace

```text
crates/
  core/       reusable application/domain logic
  main/       binary entry point
packaging/
  arch/
    local/    package built from the current working tree
    ci/       package built from a version tag
tools/sh/     local source archive and makepkg driver
```

The package installs the compiled Rust binary as `/usr/bin/skeleton`, its
README under `/usr/share/doc/skeleton/`, and the license under
`/usr/share/licenses/skeleton/`.

## Documentation

- [DEVELOPMENT.md](DEVELOPMENT.md) — build, checksums, signing, releases
- [CONTRIBUTING.md](CONTRIBUTING.md) — how to contribute
- [SECURITY.md](SECURITY.md) — reporting vulnerabilities

## License

SPDX: `GPL-3.0-only`. See [LICENSE](LICENSE).
