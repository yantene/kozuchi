// cf. https://mametter.hatenablog.com/entry/20180130/p1

use std::cmp::Ordering;

const U16_SIZE: usize = u16::MAX as usize + 1;

#[derive(Copy, Clone, Debug, PartialEq)]
enum LS {
    L,
    S,
}

fn classify_ls_types(block: &Vec<u16>) -> Vec<LS> {
    let mut result = vec![LS::S];

    for index in (0..(block.len() - 1)).rev() {
        match block[index].cmp(&block[index + 1]) {
            Ordering::Less => result.insert(0, LS::S),
            Ordering::Greater => result.insert(0, LS::L),
            Ordering::Equal => result.insert(0, result[0]),
        }
    }

    result
}

fn is_lms(ls_block: &Vec<LS>, index: usize) -> bool {
    if index == 0 {
        return false;
    }

    ls_block[index] == LS::S && ls_block[index - 1] == LS::L
}

fn calc_bins(block: &Vec<u16>) -> [usize; U16_SIZE as usize + 1] {
    let mut bins = [0; U16_SIZE as usize + 1];
    for byte in block {
        bins[*byte as usize + 1] += 1;
    }
    for i in 1..(U16_SIZE as usize + 1) {
        bins[i] += bins[i - 1];
    }

    bins
}

fn range_by_bins(bins: &[usize; U16_SIZE as usize + 1], byte: usize) -> (usize, usize) {
    (bins[byte], bins[byte + 1] - 1)
}

fn populate_bins_with_lms_index_from_back(
    suffix_array: &mut Vec<Option<usize>>,
    block: &Vec<u16>,
    bins: &[usize; U16_SIZE as usize + 1],
    seed: &Vec<usize>,
) {
    let mut count = vec![0; U16_SIZE as usize + 1];

    for lms_index in seed.iter().rev() {
        let byte = block[*lms_index] as usize;
        let (_start_of_bin, end_of_bin) = range_by_bins(&bins, byte);

        suffix_array[end_of_bin - count[byte]] = Some(*lms_index);
        count[byte] += 1;
    }
}

fn fill_bins_forward(
    suffix_array: &mut Vec<Option<usize>>,
    block: &Vec<u16>,
    bins: &[usize; U16_SIZE as usize + 1],
    ls_block: &Vec<LS>,
) {
    let mut count = vec![0; U16_SIZE as usize + 1];

    for sa_index in 0..suffix_array.len() {
        let index = suffix_array[sa_index];

        if index.is_none() || index == Some(0) {
            continue;
        }

        if ls_block[index.unwrap() - 1] == LS::L {
            let byte = block[index.unwrap() - 1] as usize;
            let (start_of_bin, _end_of_bin) = range_by_bins(&bins, byte);

            suffix_array[start_of_bin + count[byte]] = Some(index.unwrap() - 1);
            count[byte] += 1;
        }
    }
}

fn fill_bins_backward(
    suffix_array: &mut Vec<Option<usize>>,
    block: &Vec<u16>,
    bins: &[usize; U16_SIZE as usize + 1],
    ls_block: &Vec<LS>,
) {
    let mut count = vec![0; U16_SIZE as usize + 1];

    for sa_index in (0..suffix_array.len()).rev() {
        let index = suffix_array[sa_index];

        if index.is_none() || index == Some(0) {
            continue;
        }

        if ls_block[index.unwrap() - 1] == LS::S {
            let byte = block[index.unwrap() - 1] as usize;
            let (_start_of_bin, end_of_bin) = range_by_bins(&bins, byte);

            suffix_array[end_of_bin - count[byte]] = Some(index.unwrap() - 1);
            count[byte] += 1;
        }
    }
}

fn induced_sort(block: &Vec<u16>, ls_types: &Vec<LS>, seed: &Vec<usize>) -> Vec<usize> {
    let bins = calc_bins(&block);

    let mut suffix_array: Vec<Option<usize>> = vec![None; block.len()];

    populate_bins_with_lms_index_from_back(&mut suffix_array, &block, &bins, &seed);
    fill_bins_forward(&mut suffix_array, &block, &bins, &ls_types);
    fill_bins_backward(&mut suffix_array, &block, &bins, &ls_types);

    suffix_array.iter().map(|e| e.unwrap()).collect::<Vec<_>>()
}

fn detect_seed(block: &Vec<u16>, ls_types: &Vec<LS>, pseudo_seed: &Vec<usize>) -> Vec<usize> {
    let pseudo_suffix_array = induced_sort(block, ls_types, pseudo_seed);

    let pseudo_lms_indices = pseudo_suffix_array
        .iter()
        .filter(|&i| is_lms(ls_types, *i))
        .map(|&i| i)
        .collect::<Vec<_>>();

    let mut num = 0;
    let mut nums = vec![Option::None; block.len()];
    nums[pseudo_lms_indices[0]] = Some(0);

    for cons in pseudo_lms_indices.windows(2) {
        let (i, j) = (cons[0], cons[1]);

        let mut diff = false;

        for block_index in 0..block.len() {
            if block[i + block_index] != block[j + block_index]
                || is_lms(ls_types, i + block_index) != is_lms(ls_types, j + block_index)
            {
                diff = true;
                break;
            } else if block_index > 0
                && (is_lms(ls_types, i + block_index) || is_lms(ls_types, j + block_index))
            {
                break;
            }
        }

        if diff {
            num += 1
        }

        nums[j] = Some(num);
    }

    let nums = nums
        .iter()
        .filter(|&e| e.is_some())
        .map(|e| e.unwrap() as u16)
        .collect::<Vec<_>>();

    let new_suffix_array = if num + 1 < nums.len() {
        construct_suffix_array_by_induced_sorting(&nums)
    } else {
        let mut new_suffix_array = vec![0; nums.len()];
        for (index, i) in nums.iter().enumerate() {
            new_suffix_array[*i as usize] = index as usize;
        }
        new_suffix_array
    };

    new_suffix_array
        .iter()
        .map(|i| pseudo_seed[*i])
        .collect::<Vec<_>>()
}

pub fn construct_suffix_array_by_induced_sorting(block: &Vec<u16>) -> Vec<usize> {
    let ls_types = classify_ls_types(&block);

    let lms_indices = (1..ls_types.len())
        .filter(|&i| is_lms(&ls_types, i))
        .collect();

    let seed = detect_seed(&block, &ls_types, &lms_indices);

    induced_sort(&block, &ls_types, &seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_ls_types() {
        let block = b"mmiissiissiippii\x00".map(|c| c as u16).to_vec();

        let result = classify_ls_types(&block);

        let (l, s) = (LS::L, LS::S);

        assert_eq!(
            result,
            vec![l, l, s, s, l, l, s, s, l, l, s, s, l, l, l, l, s]
        );
    }

    #[test]
    fn test_construct_suffix_array_by_induced_sorting1() {
        let block = b"mmiissiissiippii\x00".map(|c| c as u16).to_vec();

        let suffix_array = construct_suffix_array_by_induced_sorting(&block);

        assert_eq!(
            suffix_array,
            vec![16, 15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4],
        )
    }

    #[test]
    fn test_construct_suffix_array_by_induced_sorting2() {
        let block = b"BABAC\x00".map(|c| c as u16).to_vec();

        let suffix_array = construct_suffix_array_by_induced_sorting(&block);

        assert_eq!(suffix_array, vec![5, 1, 3, 0, 2, 4],)
    }
}
