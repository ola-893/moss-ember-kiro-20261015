# Moss & Ember

A fictional neighbourhood café and bakery website, built as a demo of the
[Fission](https://fission.rs) static-site framework.

> **This is a demo site.** Moss & Ember is not a real business. All addresses,
> hours, prices and names are fictional and should not be relied on.

## What it is

Three-page static site — Home, Menu and Visit — deployed to GitHub Pages via
GitHub Actions. Authored entirely in Rust using Fission 0.14.1 with the
`static-site` target.

- **Home** — hero copy, daily specials strip, intro band and CTA.
- **Menu** — coffee, pastries, light bites and lunch sections with prices.
- **Visit** — opening hours, address, transport, parking and accessibility.

## Live site

<https://ola-893.github.io/moss-ember-kiro-20261015/>

## Build locally

Requires Rust (stable), the Fission CLI 0.14.1 and Linux font/pkg-config
headers when building on Linux.

```sh
# Install CLI once
cargo install cargo-fission --version 0.14.1 --locked

# Check and package
fission site check --project-dir . --release
fission package --project-dir . --target static-site --format static --release

# Output
# target/fission/release/static-site/static/index.html
```

## Deploy

Push to `main`. The bundled workflow (`.github/workflows/fission-pages.yml`)
installs the CLI, checks, packages and deploys to GitHub Pages automatically.

## Framework

Built with [Fission](https://fission.rs) — a Rust UI framework for desktop,
mobile and static web.
