use std::{fs::File, io::Read, io::Write, iter::Peekable};

/**
 * Take run length from bytes.
 */
fn take_run_length(input: &mut Peekable<std::io::Bytes<std::fs::File>>) -> Option<(u8, u8)> {
    let byte = match input.next() {
        Some(Ok(byte)) => byte,
        _ => return None,
    };

    let mut run_length: u8 = 0;
    while input.peek().is_some() && &byte == input.peek().unwrap().as_ref().unwrap() {
        run_length += 1;
        input.next();

        if run_length == 255 {
            break;
        }
    }

    return Some((byte, run_length));
}

/**
 * Write block run lengths to compressed file.
 */
fn write_block_run_lengths(block_run_lengths: &mut [u8; 256], compressed_file: &mut std::fs::File) {
    for index in 0..=255 {
        if block_run_lengths[index] != 0 {
            compressed_file
                .write(&[block_run_lengths[index], index as u8])
                .unwrap();

            block_run_lengths[index] = 0;
        }
    }
    compressed_file.write(&[0]).unwrap();
}

/**
 * Write block bytes to compressed file.
 */
fn write_block_bytes(
    block_bytes: &mut [u8; 256],
    compressed_file: &mut std::fs::File,
    limit: Option<u8>,
) {
    compressed_file
        .write(&block_bytes[0..=(limit.unwrap_or(255) as usize)])
        .unwrap();
}

pub fn run(input_file_path: &str, output_file_path: &str) {
    let input_file = File::open(input_file_path).unwrap();
    let mut output_file = File::create(output_file_path).unwrap();

    let mut input = input_file.bytes().peekable();

    let mut block_index: u8 = 0;
    let mut block_bytes: [u8; 256] = [0; 256];
    let mut block_run_lengths: [u8; 256] = [0; 256];

    while input.peek().is_some() {
        let (byte, run_length) = take_run_length(&mut input).unwrap();

        block_bytes[block_index as usize] = byte;
        block_run_lengths[block_index as usize] = run_length;
        block_index = block_index.wrapping_add(1);

        if block_index == 0 {
            write_block_run_lengths(&mut block_run_lengths, &mut output_file);
            write_block_bytes(&mut block_bytes, &mut output_file, None);
        }
    }

    if block_index != 0 {
        write_block_run_lengths(&mut block_run_lengths, &mut output_file);
        write_block_bytes(&mut block_bytes, &mut output_file, Some(block_index - 1));
    }
}
