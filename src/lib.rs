extern crate minimp3_sys as ffi;
extern crate slice_deque;

use slice_deque::SliceDeque;
use std::io::{self, Read};
use std::marker::Send;
use std::mem;

mod error;
pub use error::Error;

/// Maximum number of samples present in a MP3 frame.
pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 15;
const REFILL_TRIGGER: usize = MAX_SAMPLES_PER_FRAME * 8;

/// A MP3 decoder which consumes a reader and produces [`Frame`]s.
///
/// [`Frame`]: ./struct.Frame.html
pub struct Decoder<R> {
    reader: R,
    buffer: SliceDeque<u8>,
    decoder: Box<ffi::mp3dec_t>,
}

// Explicitly impl [Send] for [Decoder]s. This isn't a great idea and should probably be removed in the future.
// The only reason it's here is that [SliceDeque] doesn't implement [Send] (since it uses raw pointers internally),
// even though it's safe to send it across thread boundaries.
unsafe impl<R: Send> Send for Decoder<R> {}

/// A MP3 frame, owning the decoded audio of that frame.
pub struct Frame {
    /// The decoded audio held by this frame. Channels are interleaved.
    pub data: Vec<i16>,
    /// This frame's sample rate in hertz.
    pub sample_rate: i32,
    /// The number of channels in this frame.
    pub channels: usize,
    /// MPEG layer used by this file.
    pub layer: usize,
    /// Current bitrate as of this frame, in kb/s.
    pub bitrate: i32,
}

impl<R> Decoder<R>
where
    R: Read,
{
    /// Creates a new decoder, consuming the `reader`.
    pub fn new(reader: R) -> Decoder<R> {
        let mut minidec = unsafe { Box::new(mem::zeroed()) };
        unsafe { ffi::mp3dec_init(&mut *minidec) }

        Decoder {
            reader,
            buffer: SliceDeque::with_capacity(BUFFER_SIZE),
            decoder: minidec,
        }
    }

    /// Reads a new frame from the internal reader. Returns a [`Frame`] if one was found,
    /// or, otherwise, an `Err` explaining why not.
    ///
    /// [`Frame`]: ./struct.Frame.html
    pub fn next_frame(&mut self) -> Result<Frame, Error> {
        loop {
            // Keep our buffers full
            let bytes_read = if self.buffer.len() < REFILL_TRIGGER {
                Some(self.refill()?)
            } else {
                None
            };

            match self.decode_frame() {
                Ok(frame) => return Ok(frame),
                // Don't do anything if we didn't have enough data or we skipped data,
                // just let the loop spin around another time.
                Err(Error::InsufficientData) | Err(Error::SkippedData) => {
                    // If there are no more bytes to be read from the file, return EOF
                    if let Some(0) = bytes_read {
                        return Err(Error::Eof);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub fn reader(&self) -> &R {
        &self.reader
    }

    pub fn reader_mut(&mut self) -> &mut R {
        &mut self.reader
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
            unsafe {
                pcm.set_len(samples * frame_info.channels as usize);
            }
        }

        let frame = Frame {
            data: pcm,
            sample_rate: frame_info.hz,
            channels: frame_info.channels as usize,
            layer: frame_info.layer as usize,
            bitrate: frame_info.bitrate_kbps,
        };

        let current_len = self.buffer.len();
        self.buffer
            .truncate_front(current_len - frame_info.frame_bytes as usize);

        if samples == 0 {
            if frame_info.frame_bytes > 0 {
                Err(Error::SkippedData)
            } else {
                Err(Error::InsufficientData)
            }
        } else {
            Ok(frame)
        }
    }

    fn refill(&mut self) -> Result<usize, io::Error> {
        let mut dat: [u8; MAX_SAMPLES_PER_FRAME * 5] = [0; MAX_SAMPLES_PER_FRAME * 5];
        let read_bytes = self.reader.read(&mut dat)?;
        self.buffer.extend(dat.iter());

        Ok(read_bytes)
    }
}
