use std::{iter::Peekable, marker::PhantomData};

pub fn inverse_transform(block: Vec<u8>, index: usize) -> Vec<u8> {
    // O(n)
    let block_with_eos = {
        let mut block_with_eos = block.iter().map(|e| *e as u16 + 1).collect::<Vec<_>>();
        block_with_eos[index] = 0;

        block_with_eos
    };

    let mut appearance_count = vec![0; block_with_eos.len()];
    let mut cum_freq = vec![0; 65536 + 1];

    // O(n)
    for (block_index, &byte) in block_with_eos.iter().enumerate() {
        appearance_count[block_index] = cum_freq[byte as usize + 1];
        cum_freq[byte as usize + 1] += 1;
    }

    // O(n)
    for cf_index in 1..cum_freq.len() {
        cum_freq[cf_index] += cum_freq[cf_index - 1];
    }

    let mut result = vec![];
    let mut cursor = index;
    // O(n)
    for _ in 0..block_with_eos.len() {
        let byte = block_with_eos[cursor];
        result.insert(0, byte);
        cursor = cum_freq[byte as usize] + appearance_count[cursor];
    }
    result.pop();

    result.iter().map(|e| (*e - 1) as u8).collect::<Vec<_>>()
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

                // size of the block
                let mut block_length = 0;
                for _ in 0..super::BYTE_WIDTH {
                    block_length = (block_length << 8) + (self.input.next()? as usize);
                }
                block_length += 1;

                // block sort index
                let mut index = 0;
                for _ in 0..super::BYTE_WIDTH {
                    index = (index << 8) + (self.input.next()? as usize);
                }

                // the block itself
                let chunk = self
                    .input
                    .by_ref()
                    .take(block_length as usize)
                    .collect::<Vec<_>>();

                self.current_chunk = inverse_transform(chunk, index);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_transform() {
        let block = vec![0u8, 1u8, 2u8, 0u8, 0u8];
        let index = 4;
        let original = inverse_transform(block, index);

        // cf. the comment in `test_transform()`
        let expected_original = vec![2u8, 0u8, 1u8, 0u8];

        assert_eq!(original, expected_original);
    }
}
