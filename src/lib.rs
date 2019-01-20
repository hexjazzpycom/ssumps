use std::mem::size_of;

pub fn find_subset_naive(input_sum: &u32, input_set: &Vec<u32>) -> Vec<u32> {
    // lazy - returns as far as find result subset, not process all input set when it not necessary
    // memory optimized - keep in memory only two rows of bit table and accumulate calculations in
    // vector result_values_map

    let bit_table_width = *input_sum as usize + 1;
    let bit_table_height: usize = input_set.len() + 1;

    let last_column_index = bit_table_width - 1;

    let mut found = false;
    let mut result_values_map: Vec<u32> = vec![0; bit_table_width];

    let mut prev_vector: Vec<bool> = vec![false; bit_table_width];
    prev_vector[0] = true;

    for row_num in 1_usize..bit_table_height {
        let mut new_vector: Vec<bool> = vec![false; bit_table_width];
        new_vector[0] = true;

        for column_index in 1_usize..bit_table_width {
            let previous_row_cell_value = input_set[row_num - 1] as usize;

            if column_index < previous_row_cell_value {
                new_vector[column_index] = prev_vector[column_index];
            } else {
                new_vector[column_index] = prev_vector[column_index]
                    || prev_vector[column_index - previous_row_cell_value];
            };
            if new_vector[column_index] & !prev_vector[column_index] {
                result_values_map[column_index] = previous_row_cell_value as u32
            };
        }
        if new_vector[last_column_index] {
            found = true;
            break;
        };
        prev_vector = new_vector;
    }

    // find numbers
    let mut result_numbers: Vec<u32> = Vec::new();
    if found {
        let mut column_index = last_column_index;
        while column_index > 1 {
            result_numbers.push(result_values_map[column_index]);
            column_index -= result_values_map[column_index] as usize;
        }
    };

    return result_numbers;
}

pub fn find_subset(raw_input_sum: &u32, input_set: &Vec<u32>) -> Vec<u32> {
    let register_size: usize = size_of::<usize>() * 8;
    let input_sum = *raw_input_sum;
    let block_count: usize = ((input_sum + 1) / register_size as u32 + 1) as usize;

    let mut prev_vector: Vec<usize> = vec![0; block_count];
    let mut result_vector: Vec<u32> = vec![0; block_count * register_size + 1];

    let exit_mask = 1usize << ((input_sum + 1) % register_size as u32 - 1);
    let exit_block = block_count - 1;
    prev_vector[0] = 1usize;

    let max_num = usize::max_value();

    let mut found = false;
    // it's necessary to increment leght of input set, because algorithm requires one additional
    // zero-row
    let rows_count = input_set.len() + 1;
    for row_num in 1_usize..rows_count {
        let current_input_set_value = input_set[row_num - 1]; // X[i]

        let mut current_vector: Vec<usize> = Vec::with_capacity(block_count);

        for current_block_num in 0_usize..block_count {
            let prev_vector_block = &prev_vector[current_block_num];
            let mut current_block = prev_vector_block.clone(); // make at first T[i, j] = T[i-1, j]

            if current_block == max_num {
                // there is no sence to make calculations if all bits in block is 1
                // because the result of calculations would be the same
                current_vector.push(current_block);
                continue;
            };

            let shift_size = current_input_set_value;
            let prev_block_num = (shift_size / (register_size as u32)) as usize;

            if prev_block_num == current_block_num {

                let prev_block_shift_position = (shift_size % (register_size as u32)) as usize;
                let shifted_block = prev_vector[0] << prev_block_shift_position;
                current_block = current_block | shifted_block;

            } else if prev_block_num < current_block_num {

                let prev_block_shift_position = (shift_size % (register_size as u32)) as usize;
                let prev_block_abs_position = current_block_num - prev_block_num - 1;

                if prev_block_shift_position == 0 {
                    current_block = current_block | prev_vector[prev_block_abs_position + 1];
                } else {
                    let mut shifted_block = prev_vector[prev_block_abs_position]
                        >> (register_size - prev_block_shift_position);
                    shifted_block = shifted_block
                        | prev_vector[prev_block_abs_position + 1] << prev_block_shift_position;
                    current_block = current_block | shifted_block;
                };

            };

            // check for bits what became 1 in this iteration
            if current_block ^ prev_vector_block != 0 {
                // means that we found new numbers
                // and now it's necessary to put intermediate calculations into result_vector
                // Unfortunately, I have no idea how to unpack numbers accordingly changed bits
                // more easer.
                let mut check_mask = 1usize;
                let new_bits_block = current_block ^ prev_vector_block;
                if result_vector[(current_block_num * register_size) as usize] == 0
                    && new_bits_block & check_mask != 0
                {
                    result_vector[(current_block_num * register_size) as usize] =
                        current_input_set_value;
                };
                for bit_position in 1_usize..register_size {
                    check_mask <<= 1;
                    if new_bits_block & check_mask != 0
                        && result_vector
                            [(current_block_num * register_size + bit_position) as usize]
                            == 0
                    {
                        result_vector
                            [(current_block_num * register_size + bit_position) as usize] =
                            current_input_set_value;
                    };
                }
            };

            // to drop unnecessary part
            // current_block = current_block & max_num;

            current_vector.push(current_block);

            if (current_block_num == exit_block) && (current_block & exit_mask > 0) {
                found = true;
            };
        }

        prev_vector = current_vector;
        if found {
            break;
        };
    }

    let mut result_numbers: Vec<u32> = Vec::with_capacity(rows_count / 2);
    if found {
        let mut column_index: usize = input_sum as usize;

        for i in 0..input_set.len() {
            if column_index <= 1 {
                break;
            };
            result_numbers.push(result_vector[column_index]);
            column_index -= result_vector[column_index] as usize;
        }
    }
    return result_numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subset_naive__subset_exists__subset_vector() {
        let input_set = vec![6, 4, 2, 1, 3, 5];
        let input_sum = 13u32;
        let output_set = find_subset_naive(&input_sum, &input_set);
        assert_eq!(output_set, vec![1, 2, 4, 6],);
    }

    #[test]
    fn test_find_subset_naive__subset_does_not_exist__empty_vector() {
        let input_set = vec![7, 11, 3, 5];
        let input_sum = 13u32;
        let output_set = find_subset_naive(&input_sum, &input_set);
        assert_eq!(output_set, Vec::new(),);
    }

    #[test]
    fn test_find_subset_naive__subset_exists_and_duplicate_values_in_input_set__subset_vector() {
        let input_set = vec![7, 11, 7, 7, 23, 3, 3, 3, 5];
        let input_sum = 45u32;
        let output_set = find_subset_naive(&input_sum, &input_set);
        assert_eq!(output_set, vec![5, 3, 23, 7, 7],);
    }

    #[test]
    fn test_find_subset_naive__subset_exists_and_zero_values_in_input_set__subset_vector() {
        let input_set = vec![0, 11, 0, 7, 2, 3, 0, 3, 13];
        let input_sum = 13u32;
        let output_set = find_subset_naive(&input_sum, &input_set);
        assert_eq!(output_set, vec![2, 11],);
    }

    #[test]
    fn test_find_subset_naive__subset_exists_and_u32_max_values_in_input_set__subset_vector() {
        let input_set = vec![u32::max_value(), 17, 11, 0, 7, 2, 3, 0, 3, u32::max_value()];
        let input_sum = 33u32;
        let output_set = find_subset_naive(&input_sum, &input_set);
        assert_eq!(output_set, vec![3, 2, 11, 17],);
    }

    #[test]
    fn test_find_subset__subset_exists__subset_vector() {
        let input_set = vec![6, 4, 2, 1, 3, 5];
        let input_sum = 13u32;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(output_set, vec![1, 2, 4, 6],);
    }

    #[test]
    fn test_find_subset__subset_does_not_exist__empty_vector() {
        let input_set = vec![7, 11, 3, 5];
        let input_sum = 13u32;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(output_set, Vec::new(),);
    }

    #[test]
    fn test_find_subset__subset_exists_and_duplicate_values_in_input_set__subset_vector() {
        let input_set = vec![7, 11, 7, 7, 23, 3, 3, 3, 5];
        let input_sum = 45u32;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(output_set, vec![5, 3, 23, 7, 7],);
    }

    #[test]
    fn test_find_subset__subset_exists_and_zero_values_in_input_set__subset_vector() {
        let input_set = vec![0, 11, 0, 7, 2, 3, 0, 3, 13];
        let input_sum = 13u32;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(output_set, vec![2, 11],);
    }

    #[test]
    fn test_find_subset__subset_exists_and_u32_max_values_in_input_set__subset_vector() {
        let input_set = vec![u32::max_value(), 17, 11, 0, 7, 2, 3, 0, 3, u32::max_value()];
        let input_sum = 33u32;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(output_set, vec![3, 2, 11, 17],);
    }

    #[test]
    fn test_find_subset__subset_exists_and_input_set_bigger_than_batch_size__subset_vector() {
        let raw_input_set = vec![0, 11, 0, 7, 2, 3, 0, 3, 4];
        let input_set: Vec<u32> = raw_input_set
            .iter()
            .cycle()
            .take(raw_input_set.len() * 130)
            .map(|x| *x)
            .collect();
        let input_sum: u32 = 13 * 130 + 1;
        let output_set = find_subset(&input_sum, &input_set);
        assert_eq!(
            output_set,
            vec![
                11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4,
                3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3,
                2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7,
                11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4,
                3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3,
                2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7,
                11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4,
                3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3,
                2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7,
                11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4,
                3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3,
                2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7,
                11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11, 4, 3, 3, 2, 7, 11
            ],
        );
    }
}
