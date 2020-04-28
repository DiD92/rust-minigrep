# Changelog

All notable changes to this project will be documented in this file.

## 0.3.0 - 2020/04/28

### Added

* Added `clap` crate dependency.
* Added `Configuration::from_clap_app()` method.

### Removed

* Removed `Configuration::new()` method.
* Removed `CaseFormat` enum and impl blocks.

## 0.2.2 - 2020/03/29

### Added

* Added this `CHANGELONG.md` file.

### Changed

* `Config::new()` now uses `std::env::Args` instead of `&[String]`.

* `Config::new()`, `minigrep::search` and `minigrep::search_case_insensitive` now use iterator contructs internally.
