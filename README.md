# [minimp3](//github.com/lieff/minimp3) Rust bindings

[![Cargo package](https://img.shields.io/crates/v/minimp3.svg)](https://crates.io/crates/minimp3)
[![Cargo package](https://img.shields.io/crates/d/minimp3.svg)](https://crates.io/crates/minimp3)
[![Build Status](https://travis-ci.org/germangb/minimp3-rs.svg?branch=master)](https://travis-ci.org/germangb/minimp3-rs)

## Usage example

```toml
# Cargo.toml

[dependencies]
minimp3 = "0.3"
```

```rust
extern crate minimp3;

use minimp3::{Decoder, Frame};

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<Error>> {
    let mut decoder = Decoder::new(File::open("audio_file.mp3")?);

    loop {
        // Keep decoding frames until EOF is reached
        let Frame { data, sample_rate, channels, .. } = decoder.next_frame()?;
    }

    unreachable!()
}
```
