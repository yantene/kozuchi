use std::marker::PhantomData;

use super::suffix_array::construct_suffix_array_by_induced_sorting;

fn shift_and_append_eos(block: &Vec<u8>) -> Vec<u16> {
    let mut block_with_eos = block.iter().map(|e| *e as u16 + 1).collect::<Vec<_>>();
    block_with_eos.push(0);

    block_with_eos
}

pub fn transform(block: Vec<u8>) -> (Vec<u8>, usize, usize) {
    let block_with_eos = shift_and_append_eos(&block);
    let indices = construct_suffix_array_by_induced_sorting(&block_with_eos);

    let mut eos_index = 0;

    let sorted = indices
        .iter()
        .enumerate()
        .map(|(seq, index)| {
            if *index == 0 {
                // EOS
                eos_index = seq;
                return 0;
            };

            *block.get((*index + block.len() - 1) % block.len()).unwrap()
        })
        .collect::<Vec<_>>();

    let index = indices.iter().position(|i| *i == 0).unwrap();

    (sorted, index, eos_index)
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
                    .take(2usize.pow(8 * super::BYTE_WIDTH as u32) - 1) // -1 for EOS
                    .collect::<Vec<_>>();

                if chunk.len() == 0 {
                    return None;
                }

                let (transformed_block, index, eos_index) = transform(chunk);

                self.current_chunk = vec![];

                // the EOS byte index
                for octet in (0..super::BYTE_WIDTH).rev() {
                    self.current_chunk.push((eos_index >> (8 * octet)) as u8);
                }

                // size of the block
                for octet in (0..super::BYTE_WIDTH).rev() {
                    self.current_chunk
                        .push(((transformed_block.len() - 1) >> (8 * octet)) as u8);
                }

                // block sort index
                for octet in (0..super::BYTE_WIDTH).rev() {
                    self.current_chunk.push((index >> (8 * octet)) as u8);
                }

                // the block itself
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
        let block = vec![2u8, 0u8, 1u8, 0u8];
        let (sorted, index, eos_index) = transform(block);

        // $: EOS           sorted
        //                    v
        // 0| ?: [$, 2, 0, 1, 0]
        // 1| 3: [0, $, 2, 0, 1]
        // 2| 1: [0, 1, 0, $, 2]
        // 3| 2: [1, 0, $, 2, 0]
        // 4| 0: [2, 0, 1, 0, $] <- index
        let expected_sorted = vec![0u8, 1u8, 2u8, 0u8, 0u8];
        let expected_index = 4;
        let expected_eos_index = 4;

        assert_eq!(sorted, expected_sorted);
        assert_eq!(index, expected_index);
        assert_eq!(eos_index, expected_eos_index);
    }
}
