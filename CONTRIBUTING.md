# Contributing to ISO 17442 Crates

## Coding Style

Part of submitting a PR to this repository is ensuring that the formatting is correct.

### Automated Checks

The easiest part of ensuring the style guide is followed is running the following utilities, which are checked for every PR:

- `rustfmt`: Reformats the code. If the repo is "dirty" after this has been run, the PR cannot be merged.
- `cargo clippy`: An in-depth checking utility that will look for code which the authors (The Rust Foundation) think are not idiomatic rust. In practice this is a lot like PEP-8.

### Rust's Style Guide

The Rust Foundation has a [WIP style guide](https://doc.rust-lang.org/1.0.0/style/style/README.html), and we should follow it's recommendations unless there's a good reason not to:

- [Avoid `use *`, except in tests](https://doc.rust-lang.org/1.0.0/style/style/imports.html#avoid-use-*,-except-in-tests.)
- [Prefer fully importing types/traits while module-qualifying functions](https://doc.rust-lang.org/1.0.0/style/style/imports.html#prefer-fully-importing-types/traits-while-module-qualifying-functions.)
- [Always separately bind RAII (lock) guards](https://doc.rust-lang.org/1.0.0/style/features/let.html#always-separately-bind-raii-guards.-[fixme:-needs-rfc]) -- Note that you should use brace scopes.

### Our Style Guide

In addition (and sometimes overruling) the Rust Style Guide, we have our own rules:

#### Sort your inputs

The Rust Style Guide asks developers to [sort their inputs](https://doc.rust-lang.org/1.0.0/style/style/imports.html), but in this situation the sorting is considered sub-optimal. We would prefer to sort our inputs in a similar, but distinct manner:

- `extern crate` directives (typically this is just `extern crate alloc;` in no-std crates)
- `pub use` (re-)exports
- `pub mod` exports
- `mod` definitions
- `use` imports

For example:

```rust
extern crate alloc;

pub use crate::{
    module::TypeToExport,
};
pub use dependency::TypeWereUsing;

mod module;

use dependency::SomeTypeWeUseOurselves;
```

### Re-export types when necessary

Before re-exporting types, consider is whether there is an advantage to be gained by creating a newtype wrapper, e.g.:

```rust
/// This is meant to restrict the types of math which can performed on a block
/// height (to eliminate off-by-one errors).
pub struct BlockHeight(NotZero64);

/// A way to implement serde on ExternalNonSerdeThing for others
pub struct SerdeThing(ExternalNonSerdeThing);

impl Serialize for SerdeThing { /* ... */ }
impl DeserializeOwned for SerdeThing { /* ... */ }
```

If there _is_ such an advantage, then it's usually better to wrap the types. If there isn't an advantage, then you should re-export any types you're using (or expecting your users to use).

Don't:

```rust
use other_crate::ExternalThing;

pub fn my_function(thing: ExternalThing) -> bool {
    /* ... */
}
```

Do:

```rust
pub use other_crate::ExternalThing;

pub fn my_function(thing: ExternalThing) -> bool {
    /* ... */
}
```

#### Export types at the crate level

The Rust Style Guide contains the admonition to [Reexport the most important types at the crate level](https://doc.rust-lang.org/1.0.0/style/style/organization.html#reexport-the-most-important-types-at-the-crate-level.).

Don't:

```rust
pub use crate::module::SomeType;

mod module {
    pub struct SomeType;

    // Users will have to access this using `thecrate::module::Error`?
    pub enum Error {
        Error1,
        Error2,
    }

    enum Strategy {
        Strat1(SomeType),
        Strat2,
    }
}
```

Do:

```rust
pub use crate::module::{SomeType, Error as SomeError};

mod module {
    pub struct SomeType;
    pub enum Error {
        Error1,
        Error2,
    }

    enum Strategy {
        Strat1(SomeType),
        Strat2,
    }
}
```

#### Use public modules to group functions

This is the flip side of the Rust style guide's admonition to [prefer fully importing types/traits while module-qualifying functions](https://doc.rust-lang.org/1.0.0/style/style/imports.html#prefer-fully-importing-types/traits-while-module-qualifying-functions.) and our own admonition to [export types at the crate level](#export-types-at-the-crate-level).

Types should only be exported at the top level, but if you have lots of related bare functions, consider grouping them into modules exposed via a `pub mod`.

Don't:

```rust
pub struct SomeType;

pub fn foonary_frobnicate() -> bool {
    /* ... */
}

pub fn foonary_widgify() -> SomeType {
    /* ... */
}
```

Don't:

```rust
mod foonary {
    pub struct SomeType;

    pub fn frobnicate() -> SomeType {
        /* ... */
    }

    pub fn widgify() -> SomeType {
        /* ... */
    }
}

pub use crate::foonary::{
    SomeType,
    frobnicate as foonary_frobnicate,
    widgify as foonary_widgify,
};
```

Do:

```rust
pub use crate::foonary::SomeType;

pub mod foonary {
    pub(crate) struct SomeType;

    pub fn frobnicate() -> SomeType {
        /* ... */
    }

    pub fn widgify() -> SomeType {
        /* ... */
    }
}
```

#### Avoid Manual Drops

Earlier there is a reference to the Rust style guide item "[Always separately bind RAII (lock) guards](https://doc.rust-lang.org/1.0.0/style/features/let.html#always-separately-bind-raii-guards.-[fixme:-needs-rfc])." This rule should be modified slightly to indicate the use of scoping for the critical section, and expanded to be a general admonition against the use of `core::drop()` in favor of `{}`-braced scopes.

Don't:

```rust
fn use_mutex(m: sync::mutex::Mutex<int>) {
    let guard = m.lock();
    do_work(guard);
    drop(guard); // unlock the lock
    // do other work
}
```

Do:

```rust
fn use_mutex(m: sync::mutex::Mutex<int>) {
    {
        let guard = m.lock();
        do_work(guard);
    } // unlocking will happen automatically when the lock falls out of scope
    // do other work
}
```
