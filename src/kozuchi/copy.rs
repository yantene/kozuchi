use std::{fs::File, io};

use super::Kozuchi;

pub struct Copy;

fn copy(input_file_path: &str, output_file_path: &str) {
    let mut input_file = File::open(input_file_path).unwrap();
    let mut output_file = File::create(output_file_path).unwrap();

    io::copy(&mut input_file, &mut output_file).unwrap();
}

impl Kozuchi for Copy {
    fn compress(&self, input_file_path: &str, output_file_path: &str) {
        copy(input_file_path, output_file_path);
    }

    fn decompress(&self, input_file_path: &str, output_file_path: &str) {
        copy(input_file_path, output_file_path);
    }
}
