# [minimp3](//github.com/lieff/minimp3) Rust bindings

[![Cargo package](https://img.shields.io/crates/v/minimp3.svg)](https://crates.io/crates/minimp3)
[![Cargo package](https://img.shields.io/crates/d/minimp3.svg)](https://crates.io/crates/minimp3)
[![Build Status](https://travis-ci.org/germangb/minimp3-rs.svg?branch=master)](https://travis-ci.org/germangb/minimp3-rs)

## Usage example

```toml
# Cargo.toml

[dependencies]
minimp3 = "0.5"
```

```rust
use minimp3::{Decoder, Frame, Error};

use std::fs::File;

fn main() {
    let mut decoder = Decoder::new(File::open("audio_file.mp3").unwrap());

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
}
```

## Async I/O

The decoder can be used with Tokio via the `async_tokio` feature flag.

```toml
# Cargo.toml

[dependencies]
minimp3 = { version = "0.4", features = ["async_tokio"] }

# tokio runtime
tokio = {version = "0.2", features = ["full"] }
```

```rust
use minimp3::{Decoder, Frame, Error};

use tokio::fs::File;

#[tokio::main]
async fn main() {
    let file = File::open("minimp3-sys/minimp3/vectors/M2L3_bitrate_24_all.bit").await.unwrap();
    let mut decoder = Decoder::new(file);

    loop {
        match decoder.next_frame_future().await {
            Ok(Frame {
                   data,
                   sample_rate,
                   channels,
                   ..
               }) => println!("Decoded {} samples", data.len() / channels),
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
}
```
