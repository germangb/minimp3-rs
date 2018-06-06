extern crate minimp3;

use minimp3::Decoder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<Error>> {
    let mut decoder = Decoder::new(File::open(
        "minimp3-sys/minimp3/vectors/M2L3_bitrate_24_all.bit",
    )?);

    loop {
        let frame = decoder.next_frame()?;
    }

    Ok(())
}
