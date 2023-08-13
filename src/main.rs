use clap::{Arg, Command};

pub mod kozuchi;

const NAME: &str = "Kozuchi";
const VERSION: &str = "0.1";
const AUTHOR: &str = "Shuhei YOSHIDA <contact@yantene.net>";
const ABOUT: &str = "A magic hammer that can compresses and decompresses files";
const EXTENSION: &str = "kozuchi";

fn main() {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommand(
            Command::new("compress")
                .about("Compress files")
                .arg(Arg::new("output").short('o'))
                .arg(Arg::new("file").required(true).num_args(1)),
        )
        .subcommand(
            Command::new("decompress")
                .about("Decompress files")
                .arg(Arg::new("output").short('o'))
                .arg(Arg::new("file").required(true).num_args(1)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("compress", sub_m)) => {
            let input_file_path = sub_m.get_one::<String>("file").unwrap();
            let default_output_file_path = format!("{}.{}", input_file_path, EXTENSION);
            let output_file_path = sub_m
                .get_one::<String>("output")
                .unwrap_or(&default_output_file_path);

            kozuchi::Kozuchi.compress(input_file_path, &output_file_path);
        }
        Some(("decompress", sub_m)) => {
            let input_file_path = sub_m.get_one::<String>("file").unwrap();
            let default_output_file_path = if input_file_path.ends_with(&format!(".{}", EXTENSION))
            {
                input_file_path
                    .trim_end_matches(&format!(".{}.decompressed", EXTENSION))
                    .to_string()
            } else {
                format!("{}.{}", input_file_path, EXTENSION)
            };
            let output_file_path = sub_m
                .get_one::<String>("output")
                .unwrap_or(&default_output_file_path);

            kozuchi::Kozuchi.decompress(input_file_path, &output_file_path);
        }
        _ => {}
    }
}
