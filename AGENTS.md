# AGENTS.md

This file provides guidance to AI agents when working with code in this repository.

## Overview

Cargo workspace for ISO 17442 Legal Entity Identifiers (LEIs). Currently a single crate, `iso17442-types` (`types/`), which is `#![no_std]`, optionally `alloc`, edition 2024, MSRV 1.87.0. All crates share one version (`cargo-release` with `shared-version = true`; releases go through release-plz).

## Commands

```sh
cargo build --workspace --no-default-features            # no-std, no-alloc build (also CI-tested: alloc / serde / alloc,serde)
cargo clippy --all --all-features -- -D warnings         # what CI runs
cargo fmt --all -- --check
cargo nextest run --workspace                            # unit tests (CI uses `--profile ci`, see .config/nextest.toml)
cargo test --workspace --doc                             # doctests (nextest does not run these)
cargo nextest run -p iso17442-types <test_name_substring> # single test
cargo doc --workspace --all-features --no-deps           # CI builds docs on nightly
cargo deny check                                         # advisories, bans, licenses, sources (deny.toml)
cargo nono check --package iso17442-types                # verifies the dependency tree stays no_std
prek run --all-files                                     # all git hooks in prek.toml
```

CI builds and tests on 1.87.0, stable, and beta; code must compile on the MSRV.

## Architecture (`types/`)

- `lib.rs` holds everything core: a `const fn validate()` implementing the ISO 7064 mod-97 check-digit algorithm, the `Error` enum, and two types:
  - `lei` — an unsized borrowed type wrapping `[u8]` (the `&str` analogue), built via `ref-cast`'s `#[ref_cast_custom]`.
  - `Lei` — owned, stack-only `[u8; 20]` (the `String` analogue).
- Layout of an LEI: 4-char LOU (`0..4`), 14-char entity ID (`4..18`), 2 check digits (`18..20`).
- Constructors and accessors are `const fn` so LEIs can be built in `static`/`const` data — keep new APIs const-compatible where possible (this is why validation uses `while` loops and manual byte arithmetic).
- Feature-gated code lives in sibling modules named with a trailing underscore: `alloc_.rs` (`ToOwned`/`Borrow` integrations) and `serde_.rs` (serde impls, mapping `Error` variants to serde `de::Error`s). Adding an `Error` variant requires updating the matches in `serde_.rs`.
- Tests are table-driven with `yare::parameterized` inside `#[cfg(test)]` modules.
- `types/README.md` is included as the crate docs (`#![doc = include_str!("../README.md")]`), so its code blocks are doctests.

## Lints and style

The workspace denies a large set of clippy lints (`pedantic`, `missing_docs_in_private_items`, `missing_inline_in_public_items`, `absolute_paths`, `allow_attributes`, `shadow_unrelated`, `unwrap_used`, …) and `unsafe_code`. Practical consequences:

- Every item, including private ones, needs a doc comment; public functions need `#[inline]`.
- Import paths instead of writing `core::foo::Bar` inline; alias on conflict (e.g. `error::Error as CoreError`).
- Use `#[expect(...)]`, never `#[allow(...)]`; `unsafe` blocks need `#[expect(unsafe_code)]` plus a `// SAFETY:` comment.

From CONTRIBUTING.md:

- Order items: `extern crate`, then `pub use`, then `pub mod`, then `mod`, then `use`.
- Re-export types users need from dependencies; export types at the crate root; group related free functions in a `pub mod`.
- Prefer brace scopes over `drop()` for ending guard lifetimes.

Commit messages follow Conventional Commits (enforced by a `commit-msg` hook in `prek.toml`).
