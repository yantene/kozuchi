use std::{iter::Peekable, marker::PhantomData};

/**
 * Returns the rank of each item in the block.
 * The rank of an item is the number of items that are smaller than the item.
 * If there are multiple items with the same value, the item with the smaller index is considered smaller.
 */
fn rank(block: &Vec<u8>) -> Vec<usize> {
    let mut rank = vec![0; block.len()];

    for i in 0..block.len() {
        for j in 0..block.len() {
            if block[i] > block[j] || block[i] == block[j] && i > j {
                rank[i] += 1;
            }
        }
    }

    rank
}

pub fn inverse_transform(block: Vec<u8>, index: usize) -> Vec<u8> {
    let order_map = rank(&block);

    let mut result = Vec::with_capacity(block.len());
    let mut cursor = index;
    for _ in 0..block.len() {
        result.insert(0, block[cursor]);
        cursor = order_map[cursor];
    }

    result
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

                let block_length = self.input.next()? as usize + 1;
                let index = self.input.next()?;
                let chunk = self
                    .input
                    .by_ref()
                    .take(block_length as usize)
                    .collect::<Vec<_>>();

                self.current_chunk = inverse_transform(chunk, index as usize);
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
        let block = "BNN^AA|A".chars().map(|c| c as u8).collect();
        let index = 6;

        let original = inverse_transform(block, index);

        assert_eq!(
            original,
            "^BANANA|".chars().map(|c| c as u8).collect::<Vec<u8>>()
        )
    }
}
