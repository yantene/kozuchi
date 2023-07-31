mod compress;
mod decompress;

use clap::{parser::ValuesRef, Arg, Command};
use compress::compress;
use decompress::decompress;

fn main() {
    let matches = Command::new("Kozuchi")
        .version("0.1")
        .author("Shuhei YOSHIDA <contact@yantene.net>")
        .about("A magic hammer that can compresses and decompresses files")
        .subcommand(
            Command::new("compress")
                .about("Compress files")
                .arg(Arg::new("file").required(true).num_args(1..)),
        )
        .subcommand(
            Command::new("decompress")
                .about("Decompress files")
                .arg(Arg::new("file").required(true).num_args(1..)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("compress", sub_m)) => {
            let file_paths: ValuesRef<String> = sub_m.get_many("file").unwrap();
            for file_path in file_paths {
                compress(file_path);
            }
        }
        Some(("decompress", sub_m)) => {
            let file_paths: ValuesRef<String> = sub_m.get_many("file").unwrap();
            for file_path in file_paths {
                decompress(file_path);
            }
        }
        _ => {}
    }
}
