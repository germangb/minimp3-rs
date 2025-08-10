#[cfg(feature = "std")]
use {
    minimp3::{Decoder, Error, Frame},
    std::io::Cursor,
};

#[test]
#[cfg(feature = "std")]
fn decode_test_mp3() {
    let file = include_bytes!("../../../res/test.mp3");
    let file = Cursor::new(file);

    let mut decoder = Decoder::new(file);

    let sample_count_expected = 120960;
    let mut sample_count_actual = 0;
    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => {
                assert_eq!(sample_rate, 44100);
                assert_eq!(channels, 2);

                let samples = data.len() / channels;
                sample_count_actual += samples;
            }
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
    assert_eq!(sample_count_actual, sample_count_expected);
}
