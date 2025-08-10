# minimp3-sys

[![Cargo package](https://img.shields.io/crates/v/minimp3-sys.svg)](https://crates.io/crates/minimp3-sys)
[![Cargo package](https://img.shields.io/crates/d/minimp3-sys.svg)](https://crates.io/crates/minimp3-sys)

How to manually generate minimp3 bindings using [**bindgen**](https://crates.io/crates/bindgen):

```bash
bindgen --no-rustfmt-bindings minimp3.c -- -Iminimp3 > src/bindings.rs
```
