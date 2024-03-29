# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2021-04-05

# Dev Changes
* Use glium_text_rusttype instead of glium_text, to avoid dependency on Freetype
* Updated dependencies

## [0.2.1] - 2020-12-28
Internal changes only, should be functionally identical to the previous 0.2.0 release.

# Dev Changes
* Switch to using env_logger
* Use glium_text dependency from https://github.com/rschristian/glium_text
* Updated and pinned some other dependencies

## [0.2.0] - 2020-04-23
# Added
* Pressing F3 brings up a debug overlay showing blocks rendered/blocks nearby + "TPS"

# Removed
* Logging of stats that are now visible in F3 instead

# Dev Changes
* Upgraded glium to the latest version
* Upgraded all other minor dependencies to the latest version
* Some internal refactoring

## [0.1.1] - 2020-04-22
Internal changes only, should be functionally identical to the previous 0.1.0-alpha release.

# Dev Changes
* Upgraded all dependencies to their latest versions except for glium, which was upgraded to a newer version nonetheless
* Modernizing codebase and CI pipeline (Rust 2018 edition, using clippy/rustfmt, etc)

## [0.1.0-alpha] - 2020-04-20
Initial tagged release.
