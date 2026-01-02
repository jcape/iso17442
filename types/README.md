# ISO 17442 Types

[![License][license-image]][license-link]<!--
-->[![Crates.io][crate-image]][crate-link]<!--
-->[![Docs Status][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]

This crate provides `no-std`, `no-alloc` compatible data structures for use handling ISO 17442 Legal Entity IDs. The primary type is [`Lei`](crate::Lei), which is an owned (but non-heap) representation of an LEI string. For example:

```rust
use iso17442_types::Lei;
use core::str::FromStr;

const LEI_STR: &str = "YZ83GD8L7GG84979J516";

let l = Lei::from_str(LEI_STR).expect("Could not parse LEI");
let s = l.as_str();

assert_eq!(s, LEI_STR);
```

There is also an additional [`lei`](crate::lei) borrow type. This is the `&str` to `Lei`'s `String`:

```rust
use iso17442_types::{Lei, lei};
use core::str::FromStr;

const LEI_STR: &str = "YZ83GD8L7GG84979J516";

let l = lei::from_str_slice(LEI_STR).expect("Could not parse LEI");

assert_eq!(l.as_str(), LEI_STR);
```

Both of these types are fully usable in the `const` context, making them suitable for use within static data.

[license-image]: https://img.shields.io/github/license/jcape/iso17442?style=for-the-badge
[license-link]: ../LICENSE
[crate-image]: https://img.shields.io/crates/v/iso17442-types.svg?style=for-the-badge
[crate-link]: https://crates.io/crates/iso17442-types/0.3.2
[docs-image]: https://img.shields.io/docsrs/iso17442-types?style=for-the-badge
[docs-link]: https://docs.rs/crate/iso17442-types/0.3.2
[deps-image]: https://deps.rs/crate/iso17442-types/0.3.2/status.svg?style=for-the-badge
[deps-link]: https://deps.rs/crate/iso17442-types/0.3.2
