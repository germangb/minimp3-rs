extern crate minimp3_sys;

use std::io;
use std::mem;

use std::io::Write;

fn decode_frame(
    ctx: &mut minimp3_sys::mp3dec_t,
    mp3_file: &[u8],
    pcm: &mut [i16],
    frame_info: &mut minimp3_sys::mp3dec_frame_info_t,
) -> Option<usize> {
    unsafe {
        let samples = minimp3_sys::mp3dec_decode_frame(
            ctx,
            mp3_file.as_ptr(),
            mp3_file.len() as _,
            pcm.as_mut_ptr(),
            frame_info,
        );

        match frame_info.frame_bytes {
            0 => None,
            _ => Some(samples as usize),
        }
    }
}

fn main() {
    let mp3 = include_bytes!("../minimp3/vectors/M2L3_bitrate_24_all.bit");

    let mut context = unsafe { mem::zeroed() };

    unsafe { minimp3_sys::mp3dec_init(&mut context) };

    // output samples
    let mut pcm = vec![0i16; minimp3_sys::MINIMP3_MAX_SAMPLES_PER_FRAME as usize];

    // frame info
    let mut frame: minimp3_sys::mp3dec_frame_info_t = unsafe { mem::zeroed() };

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
