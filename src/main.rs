mod compress;
mod decompress;

use clap::{Arg, Command};
use compress::compress;

fn main() {
    let matches = Command::new("Kozuchi")
        .version("0.1")
        .author("Shuhei YOSHIDA <contact@yantene.net>")
        .about("A magic hammer that can compresses and decompresses files")
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
            let default_output_file_path = format!("{}.kozuchi", input_file_path);
            let output_file_path = sub_m
                .get_one::<String>("output")
                .unwrap_or(&default_output_file_path);

            compress(input_file_path, &output_file_path);
        }
        Some(("decompress", sub_m)) => {
            let input_file_path = sub_m.get_one::<String>("file").unwrap();
            let default_output_file_path = if input_file_path.ends_with(".kozuchi") {
                input_file_path.trim_end_matches(".kozuchi").to_string()
            } else {
                format!("{}.unkozuchied", input_file_path)
            };
            let output_file_path = sub_m
                .get_one::<String>("output")
                .unwrap_or(&default_output_file_path);

            compress(input_file_path, &output_file_path);
        }
        _ => {}
    }
}
