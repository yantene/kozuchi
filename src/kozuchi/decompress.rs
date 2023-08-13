use std::{fs::File, io::Write};

use super::encoders::{burrows_wheeler_transform, run_length_encoding};
use super::utils::iterate_bytes;

pub fn run(input_file_path: &str, output_file_path: &str) {
    let input_file = File::open(input_file_path).unwrap();

    let mut input = iterate_bytes(input_file);

    let mut rle_decoded = run_length_encoding::decode(&mut input);

    let bwt_decoded = burrows_wheeler_transform::decode(&mut rle_decoded);

    // Output to file.
    let mut output_file = File::create(output_file_path).unwrap();
    for byte in bwt_decoded {
        output_file.write_all(&[byte]).unwrap();
    }
    output_file.flush().unwrap();
}
