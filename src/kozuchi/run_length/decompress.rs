use std::{io::Read, io::Write, iter::Peekable};

fn take_run_lengths(input: &mut Peekable<std::io::Bytes<std::fs::File>>) -> [u8; 256] {
    let mut run_lengths = [0; 256];

    while input.peek().is_some() && &0 != input.peek().unwrap().as_ref().unwrap() {
        let run_length = input.next().unwrap().unwrap();
        let index = input.next().unwrap().unwrap();

        run_lengths[index as usize] = run_length;
    }
    input.next();

    return run_lengths;
}

pub fn run(input_file_path: &str, output_file_path: &str) {
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let mut output_file = std::fs::File::create(output_file_path).unwrap();

    let mut input = input_file.bytes().peekable();

    while input.peek().is_some() {
        let run_lengths = take_run_lengths(&mut input);

        for index in 0..=255 {
            let byte = match input.next() {
                Some(byte) => byte.unwrap(),
                None => break,
            };

            output_file
                .write(&vec![byte; run_lengths[index as usize] as usize + 1])
                .unwrap();
        }
    }
}
