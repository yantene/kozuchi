use std::{iter::Peekable, marker::PhantomData};

/**
 * Take run length from bytes.
 */
fn take_run_length<'a>(input: &mut Peekable<impl Iterator<Item = u8> + 'a>) -> Option<(u8, u8)> {
    let byte = match input.next() {
        Some(byte) => byte,
        _ => return None,
    };

    let mut run_length = 0u8;
    while input.peek().is_some() && &byte == input.peek().unwrap() {
        run_length += 1;
        input.next();

        if run_length == 255 {
            break;
        }
    }

    return Some((byte, run_length));
}

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Encoded<'a, I: Iterator<Item = u8> + 'a> {
        input: Peekable<I>,
        current_chunk: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Encoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_chunk.len() == 0 {
                if self.input.peek().is_none() {
                    return None;
                }

                let mut block_run_lengths = Vec::new();
                let mut block_bytes = Vec::new();
                for block_index in 0u8..=255u8 {
                    if self.input.peek().is_none() {
                        break;
                    }

                    let (byte, run_length) = take_run_length(&mut self.input).unwrap();

                    if run_length > 0 {
                        block_run_lengths.push(run_length);
                        block_run_lengths.push(block_index);
                    }
                    block_bytes.push(byte);
                }

                self.current_chunk = block_run_lengths.clone();
                self.current_chunk.extend([0]);
                self.current_chunk.extend(block_bytes);
            }

            Some(self.current_chunk.remove(0))
        }
    }

    Encoded {
        input: input.peekable(),
        current_chunk: Vec::new(),
        _marker: PhantomData,
    }
}
