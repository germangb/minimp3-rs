//! This example must be run with the "async_tokio" feature flag:
//!
//! ```bash
//! $ cargo run --example example_tokio --features async_tokio
//! ```
use minimp3::{Decoder, Error, Frame};

use tokio::fs::File;

#[tokio::main]
async fn main() {
    let mut decoder = Decoder::new(
        File::open("minimp3-sys/minimp3/vectors/M2L3_bitrate_24_all.bit")
            .await
            .unwrap(),
    );

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
