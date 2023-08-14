use std::marker::PhantomData;

pub fn transform(block: Vec<u8>) -> (Vec<u8>, usize) {
    let mut indices = (0..block.len()).collect::<Vec<usize>>();
    indices.sort_by(|a, b| {
        for index in 0..block.len() {
            let item_a = block.get((*a + index) % block.len()).unwrap();
            let item_b = block.get((*b + index) % block.len()).unwrap();

            if item_a < item_b {
                return std::cmp::Ordering::Less;
            } else if item_a > item_b {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    });

    let sorted = indices
        .iter()
        .map(|i| *block.get((*i + block.len() - 1) % block.len()).unwrap())
        .collect::<Vec<_>>();

    let index = indices.iter().position(|i| *i == 0).unwrap();

    (sorted, index)
}

pub fn run<'a, I: Iterator<Item = u8> + 'a>(input: &'a mut I) -> impl Iterator<Item = u8> + 'a {
    struct Encoded<'a, I: Iterator<Item = u8> + 'a> {
        input: I,
        current_chunk: Vec<u8>,
        _marker: PhantomData<&'a mut I>,
    }

    impl<'a, I: Iterator<Item = u8>> Iterator for Encoded<'a, I> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_chunk.len() == 0 {
                let chunk = self
                    .input
                    .by_ref()
                    .take(2usize.pow(8 * super::BYTE_WIDTH as u32))
                    .collect::<Vec<_>>();

                if chunk.len() == 0 {
                    return None;
                }

                let (transformed_block, index) = transform(chunk);

                self.current_chunk = vec![];
                for octet in (0..super::BYTE_WIDTH).rev() {
                    self.current_chunk
                        .push(((transformed_block.len() - 1) >> (8 * octet)) as u8);
                }
                for octet in (0..super::BYTE_WIDTH).rev() {
                    self.current_chunk.push((index >> (8 * octet)) as u8);
                }
                self.current_chunk.extend(transformed_block);
            }

            Some(self.current_chunk.remove(0))
        }
    }

    Encoded {
        input,
        current_chunk: Vec::new(),
        _marker: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let block = "^BANANA|".chars().map(|c| c as u8).collect();

        let (sorted, index) = transform(block);

        assert_eq!(
            sorted,
            "BNN^AA|A".chars().map(|c| c as u8).collect::<Vec<u8>>()
        );
        assert_eq!(index, 6);
    }
}
