use std::{iter::Peekable, marker::PhantomData};

fn take_block_run_lengths<'a>(input: &mut Peekable<impl Iterator<Item = u8> + 'a>) -> [u8; 256] {
    let mut run_lengths_map = [0u8; 256];

    while let Some(run_length) = input.next() {
        if run_length == 0 {
            break;
        }

        let index = input.next().unwrap();
        run_lengths_map[index as usize] = run_length;
    }

    return run_lengths_map;
}

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Decoded<'a, I: Iterator<Item = u8> + 'a> {
        input: Peekable<I>,
        current_chunk: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Decoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_chunk.len() == 0 {
                if self.input.peek().is_none() {
                    return None;
                }

                let run_lengths_map = take_block_run_lengths(&mut self.input);

                for index in 0u8..=255u8 {
                    let byte = match self.input.next() {
                        Some(byte) => byte,
                        None => break,
                    };

                    self.current_chunk
                        .extend(vec![byte; run_lengths_map[index as usize] as usize + 1]);
                }
            }

            Some(self.current_chunk.remove(0))
        }
    }

    Decoded {
        input: input.peekable(),
        current_chunk: Vec::new(),
        _marker: PhantomData,
    }
}
