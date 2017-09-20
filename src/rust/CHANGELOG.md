# Qork Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Eventually.

Actually, at the moment, this is more a record of my learning experience writing
a non-trivial program in Rust. The version number gets bumped at the end of every
month but since there has never been a formal release I am not really adhering to
semver yet.

## Unreleased
### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## 2017-09-30 - 0.1.3 (xxxx lines of Rust)
### Added
* First use of interior mutability, `RefCell` in Context to hold PersistentState.
* First use of `AsRef`.
* First macros written for the `ExecutionTimer` struct.
* Added `xi-rope` crate.
* Added `shellexpand` and wrote my first closure, for use as a private local function.
* Started the `Buffer` class.

## 2017-08-31 - 0.1.2 (1169 lines of Rust)
### Added
* First use of a trait, `BaseDir`.
* First use of the tools `clippy`, `rustfmt` and `loc`.
* First use of a sub-module, `fs`. Wraps the `xdg` crate functionality.
* First unit tests, for `MRUList`.
* First use of `serde`, for loading `Configuration`.
* Qork now has basic system/program info, a Context and RuntimeData.
  Those structs are reasonably well written (for a newbie).
* Created this Changelog and `loc.txt`.

### Changed
* The `Built` crate now comes from crates.io again, not the git master.

## 2017-07-31 0.1.1 (250 lines of Rust)
### Added
* My first `Drop` struct - `ExecutionTimer`.
* Switch to using VSCode (so that Geany and KDevelop stuff is really obsolete).
### Changed
* Get basic logging working (but change from `slog` to standard `log`).
* Stop worrying about Python and concentrate on Rust.

## 2017-06-30 - 0.1.0 (0 lines of Rust)
### Added
* Get a basic Makefile working, mainly for Python.
* Lots of reading!
