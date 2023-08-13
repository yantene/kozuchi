pub struct Kozuchi;

impl Kozuchi {
    pub fn compress(&self, input_file_path: &str, output_file_path: &str) {
        compress::run(input_file_path, output_file_path);
    }

    pub fn decompress(&self, input_file_path: &str, output_file_path: &str) {
        decompress::run(input_file_path, output_file_path);
    }
}

pub mod compress;
pub mod decompress;
pub mod encoders;
pub mod utils;
