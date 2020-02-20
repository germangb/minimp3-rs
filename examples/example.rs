use minimp3::{Decoder, Error, Frame};

use std::fs::File;

fn main() {
    let mut decoder =
        Decoder::new(File::open("minimp3-sys/minimp3/vectors/M2L3_bitrate_24_all.bit").unwrap());

    loop {
        match decoder.next_frame() {
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
