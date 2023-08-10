pub trait Kozuchi {
    fn compress(&self, input_file_path: &str, output_file_path: &str);
    fn decompress(&self, input_file_path: &str, output_file_path: &str);
}

pub mod copy;
pub mod run_length;
