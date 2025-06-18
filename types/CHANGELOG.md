# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/jcape/iso17442/compare/v0.2.0...v0.3.0) - 2025-06-18

### Added

- [**breaking**] implement deref for owned lei, add lou, id accessors
- inline one-liners
- [**breaking**] improve errors, add custom string ser/de.
- serde and display implementations

### Fixed

- add more serde visitor types.

### Other

- added examples to readme, more codecov
- *(ci)* build and test under all featuresets

## [0.2.0](https://github.com/jcape/iso17442/compare/v0.1.0...v0.2.0) - 2025-06-16

### Added

- [**breaking**] rework to implement as Lei/lei with Borrow/ToOwned

### Other

- initial commit
- lint and cargo-deny fixes
