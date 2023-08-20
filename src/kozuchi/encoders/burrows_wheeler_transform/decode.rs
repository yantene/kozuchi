use std::{iter::Peekable, marker::PhantomData};

/**
 * Returns the rank of each item in the block.
 * The rank of an item is the number of items that are smaller than the item.
 * If there are multiple items with the same value, the item with the smaller index is considered smaller.
 */
fn rank(block: &Vec<u16>) -> Vec<usize> {
    let mut rank = vec![0; block.len()];

    for i in 0..block.len() {
        for j in (i + 1)..block.len() {
            if block[i] > block[j] {
                rank[i] += 1;
            } else if block[i] < block[j] {
                rank[j] += 1;
            } else if i > j {
                rank[i] += 1;
            } else {
                rank[j] += 1;
            }
        }
    }

    rank
}

pub fn inverse_transform(block: Vec<u8>, index: usize, eos_index: usize) -> Vec<u8> {
    let block_with_eos = {
        let mut block_with_eos = block.iter().map(|e| *e as u16 + 1).collect::<Vec<_>>();
        block_with_eos[eos_index] = 0;

        block_with_eos
    };

    let order_map = rank(&block_with_eos);

    let mut result = vec![];
    let mut cursor = index;
    for _ in 0..block_with_eos.len() {
        result.insert(0, block_with_eos[cursor]);
        cursor = order_map[cursor];
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

                let mut eos_index = 0;
                for _ in 0..super::BYTE_WIDTH {
                    eos_index = (eos_index << 8) + (self.input.next()? as usize);
                }

                let mut block_length = 0;
                for _ in 0..super::BYTE_WIDTH {
                    block_length = (block_length << 8) + (self.input.next()? as usize);
                }
                block_length += 1;

                let mut index = 0;
                for _ in 0..super::BYTE_WIDTH {
                    index = (index << 8) + (self.input.next()? as usize);
                }
                let chunk = self
                    .input
                    .by_ref()
                    .take(block_length as usize)
                    .collect::<Vec<_>>();

                self.current_chunk = inverse_transform(chunk, index, eos_index);
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
        let eos_index = 4;
        let original = inverse_transform(block, index, eos_index);

        // cf. the comment in `test_transform()`
        let expected_original = vec![2u8, 0u8, 1u8, 0u8];

        assert_eq!(original, expected_original);
    }
}
