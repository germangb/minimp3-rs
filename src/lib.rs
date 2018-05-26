extern crate minimp3_sys as ffi;
extern crate slice_deque;

use std::io::{self, Read};
use std::marker::Send;
use std::mem;
use slice_deque::SliceDeque;

mod error;
pub use error::Error;

/// Maximum samples we will ever see in a single MP3 frame.
pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 6;

pub struct Decoder<R>
where
    R: Read,
{
    reader: R,
    buffer: SliceDeque<u8>,
    decoder: Box<ffi::mp3dec_t>,
}

// Explicitly impl [Send] for [Decoder]s. This isn't a great idea and should probably be removed in the future.
// The only reason it's here is that [SliceDeque] doesn't implement [Send] (since it uses raw pointers internally),
// even though it's safe to send it across thread boundaries.
unsafe impl<R> Send for Decoder<R> where R: Read {}

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
            buffer: SliceDeque::with_capacity(BUFFER_SIZE),
            decoder: minidec,
        }
    }

    pub fn next_frame(&mut self) -> Result<Frame, Error> {
        match self.decode_frame() {
            Ok(frame) => Ok(frame),
            Err(Error::InsufficientData) => {
                // attempt a refill
                if self.refill()? == 0 {
                    Err(Error::Eof)
                } else {
                    // if that worked, grab a new frame
                    self.next_frame()
                }
            },
            Err(Error::SkippedData) => {
                // try reading a new frame
                self.next_frame()
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

        if samples > 0 {
            unsafe {pcm.set_len(samples * frame_info.channels);}
        }

        let frame = Frame {
            data: pcm,
            sample_rate: frame_info.hz,
            channels: frame_info.channels as usize,
            layer: frame_info.layer as usize,
            bitrate: frame_info.bitrate_kbps,
        };


        let current_len = self.buffer.len();
        self.buffer.truncate_front(current_len - frame_info.frame_bytes as usize);

        if samples == 0 {
            if frame_info.frame_bytes > 0 {
                if self.buffer.len() < MAX_SAMPLES_PER_FRAME*2 {
                    Err(Error::InsufficientData)
                } else {
                    Err(Error::SkippedData)
                }
            } else {
                Err(Error::InsufficientData)
            }
        } else {
            Ok(frame)
        }
    }

    fn refill(&mut self) -> Result<usize, io::Error> {
        let mut dat: [u8; MAX_SAMPLES_PER_FRAME*2] = [0; MAX_SAMPLES_PER_FRAME*2];
        let read_bytes = self.reader.read(&mut dat)?;
        self.buffer.extend(dat.iter());

        Ok(read_bytes)
    }
}
