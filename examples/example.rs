extern crate minimp3;

use minimp3::bindgen;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::mem;

fn preload_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let file = File::open(path).unwrap();
    let buffer: Result<Vec<_>, _> = file.bytes().collect();
    buffer.unwrap()
}

fn decode_frame(ctx: &mut bindgen::mp3dec_t, mp3_file: &[u8], pcm: &mut [i16], frame_info: &mut bindgen::mp3dec_frame_info_t) -> Option<usize> {
    unsafe {
        match bindgen::mp3dec_decode_frame(ctx, mp3_file.as_ptr(), mp3_file.len() as _, pcm.as_mut_ptr(), frame_info) {
            0       => None,
            n @ _   => Some(n as usize),
        }
    }
}

fn main() {
    // read mp3 file
    let mp3_buf = preload_file("/home/germangb/Downloads/saxu.mp3");

    let mut context = unsafe { mem::zeroed() };

    println!("fs file read ({}, bytes)", mp3_buf.len());
    println!("sizeof(ctx) = {}", mem::size_of::<bindgen::mp3dec_t>());

    unsafe { bindgen::mp3dec_init(&mut context) };

    let mut pcm = vec![0i16; bindgen::MINIMP3_MAX_SAMPLES_PER_FRAME as usize];
    let mut frame: bindgen::mp3dec_frame_info_t = unsafe { mem::zeroed() };

    let mut total_samples = 0;
    let mut offset = 0usize;

    while let Some(samples) = decode_frame(&mut context, &mp3_buf[offset..], &mut pcm, &mut frame) {
        println!("frame {:?}", frame);
        offset += frame.frame_bytes as usize;
        total_samples += samples;
    }

    println!("---");
    println!("total_samples = {}", total_samples);
}
