extern crate minimp3;

use minimp3::bindgen;

use std::mem;
use std::io;

use std::io::Write;

fn decode_frame(ctx: &mut bindgen::mp3dec_t, mp3_file: &[u8], pcm: &mut [i16], frame_info: &mut bindgen::mp3dec_frame_info_t) -> Option<usize> {
    unsafe {
        let samples = bindgen::mp3dec_decode_frame(ctx, mp3_file.as_ptr(), mp3_file.len() as _, pcm.as_mut_ptr(), frame_info);

        match frame_info.frame_bytes {
            0       => None,
            _   => Some(samples as usize),
        }
    }
}

fn main() {
    let mp3 = include_bytes!("../minimp3/vectors/M2L3_bitrate_24_all.bit");

    let mut context = unsafe { mem::zeroed() };

    unsafe { bindgen::mp3dec_init(&mut context) };

    // output samples
    let mut pcm = vec![0i16; bindgen::MINIMP3_MAX_SAMPLES_PER_FRAME as usize];

    // frame info
    let mut frame: bindgen::mp3dec_frame_info_t = unsafe { mem::zeroed() };

    let mut offset = 0usize;
    let mut stdout = io::stdout();

    while let Some(samples) = decode_frame(&mut context, &mp3[offset..], &mut pcm, &mut frame) {
        //eprintln!("frame {:?}", frame);
        offset += frame.frame_bytes as usize;

        unsafe {
            use std::slice;

            let slice = slice::from_raw_parts(pcm.as_ptr() as _, samples * 2);
            stdout.write(slice).unwrap();
        }
    }
}
