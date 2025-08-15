# minimp3-rs

Rust bindings and high-level wrapper for the minimp3 library.

> [!CAUTION]
> This crate is not recommended for new projects due to multiple memory
> unsoundness issues and the availability of mature, safe Rust alternatives.
> Consider using fully Rust-based libraries instead, such as:
>
> - [symphonia](https://crates.io/crates/symphonia)
> - [nanomp3](https://crates.io/crates/nanomp3)

> [!IMPORTANT]
> Maintainership Update: I (@phip1611) have taken over maintainership from
> @germangb to prevent this crate from bit rotting. My effort will be limited,
> so I welcome and encourage community contributions!

## Crate Overview

This is a Cargo workspace consisting of multiple crates:

- [minimp3](./crates/minimp3)
- [minimp3-sys](./crates/minimp3-sys)

