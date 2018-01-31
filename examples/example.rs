extern crate minimp3;

use minimp3::bindgen;

fn main() {
        let mut bind_context = bindgen::mp3dec_t {
            mdct_overlap: [[0.0; 288usize]; 2usize],
            qmf_state: [0.0; 960usize],
            reserv: 0,
            free_format_bytes: 0,
            header: [0; 4usize],
            reserv_buf: [0; 511usize],
        };

        unsafe { bindgen::mp3dec_init(&mut bind_context) };
}
