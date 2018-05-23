extern crate minimp3_sys as ffi;

use std::io::{self, Read};
use std::mem;

mod error;
pub use error::Error;

/// Maximum samples we will ever see in a single MP3 frame.
pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 3;

pub struct Decoder<R>
where
    R: Read,
{
    reader: R,
    buffer: Box<[u8; BUFFER_SIZE]>,
    decoder: Box<ffi::mp3dec_t>,
}

pub struct Frame {
    /// The raw data held by this frame.
    pub data: Vec<i16>,
    /// This frame's sample rate (Hz)
    pub sample_rate: i32,
    /// The number of channels in this frame.
    pub channels: usize,
    /// MPEG layer used by this file
    pub layer: usize,
    /// Current bitrate as of this frame (kb/s)
    pub bitrate: i32,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(reader: R) -> Decoder<R> {
        let mut minidec = unsafe { Box::new(mem::zeroed()) };
        unsafe { ffi::mp3dec_init(&mut *minidec) }

        Decoder {
            reader,
            buffer: Box::new([0; BUFFER_SIZE]),
            decoder: minidec,
        }
    }

    pub fn next_frame(&mut self) -> Result<Frame, Error> {
        match self.decode_frame() {
            Ok(frame) => Ok(frame),
            Err(Error::EmptyBuffer) => {
                // attempt a refill
                if self.refill()? == 0 {
                    Err(Error::Eof)
                } else {
                    // if that worked, grab a new frame
                    self.next_frame()
                }
            },
            Err(e) => Err(e),
        }
    }

    fn decode_frame(&mut self) -> Result<Frame, Error> {
        let mut frame_info = unsafe { mem::zeroed() };
        let mut pcm = Vec::with_capacity(MAX_SAMPLES_PER_FRAME);
        let samples: usize = unsafe {
            ffi::mp3dec_decode_frame(
                &mut *self.decoder,
                self.buffer.as_ptr(),
                self.buffer.len() as _,
                pcm.as_mut_ptr(),
                &mut frame_info,
            ) as _
        };

        let frame = Frame {
            data: pcm,
            sample_rate: frame_info.hz,
            channels: frame_info.channels as usize,
            layer: frame_info.layer as usize,
            bitrate: frame_info.bitrate_kbps,
        };

        if samples == 0 {
            Err(Error::EmptyBuffer)
        } else {
            Ok(frame)
        }
    }

    fn refill(&mut self) -> Result<usize, io::Error> {
        self.reader.read(&mut *self.buffer)
    }
}
