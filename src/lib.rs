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

pub fn find_subset(raw_input_sum: &u32, raw_input_set: &Vec<u32>) -> Vec<u32> {
    let register_size = 64_u64;
    let input_sum = *raw_input_sum as u64;
    let input_set: Vec<u64> = raw_input_set.into_iter().map(|x| *x as u64).collect();
    let rows_count = input_set.len() as u64 + 1;
    let block_count: u64 = (input_sum + 1) / register_size + 1;

    println!("calc_sum_v3");
    println!("s = {}", input_sum);
    println!("n = {}", input_set.len());

    let mut prev_vector: Vec<u64> = vec![0; block_count as usize];
    let mut result_vector: Vec<u64> = vec![0; (block_count * register_size + 1) as usize];

    let mut found = false;
    let mut maxrow = 0;

    let exit_mask = 1u64 << ((input_sum + 1) % register_size - 1);
    let exit_block = block_count - 1;
    prev_vector[0] = 1u64;

    let mut max_num = 1u64;
    for i in 1..register_size {
        max_num = max_num | 1u64 << i;
    }

    let mut processed_rows = 0u64;
    for i in 1_u64..rows_count {
        processed_rows += 1;
        if maxrow > 0 {
            break;
        }
        let current_input_set_value = input_set[i as usize - 1];
        let matrix_row = i + 1;
        let mut current_vector: Vec<u64> = Vec::with_capacity(block_count as usize);
        for current_block_num in 0_u64..block_count {

            let mut current_block = prev_vector[current_block_num as usize].clone();

            if current_block == max_num {
                current_vector.push(current_block);
                continue;
                if current_block_num == exit_block {
                    found = true;
                    break;
                } else {
                    continue;
                };
            };

            let mut shifted_block = 0_u64;
            let shift_size = current_input_set_value;
            let prev_block_shift_position = shift_size % register_size;

            if shift_size / register_size == current_block_num {
                shifted_block = prev_vector[0] << prev_block_shift_position;
            } else if shift_size / register_size > current_block_num {
                shifted_block = 0u64;
            } else {
                let prev_block_abs_position = current_block_num - shift_size / register_size - 1;
                shifted_block = prev_vector[prev_block_abs_position as usize]
                    >> (register_size - prev_block_shift_position);
                shifted_block = shifted_block
                    | prev_vector[(prev_block_abs_position + 1) as usize]
                        << prev_block_shift_position;
            };
            current_block = current_block | shifted_block;

            if current_block ^ prev_vector[current_block_num as usize] != 0 {
                // means that we found new numbers
                let mut check_mask = 1u64;
                let new_bytes_block = current_block ^ prev_vector[current_block_num as usize];
                if result_vector[(current_block_num * register_size) as usize] == 0
                    && new_bytes_block & check_mask != 0
                {
                    result_vector[(current_block_num * register_size) as usize] =
                        current_input_set_value;
                };
                for byte_position in 1_u64..register_size {
                    check_mask <<= 1;
                    if new_bytes_block & check_mask != 0
                        && result_vector
                            [(current_block_num * register_size + byte_position) as usize]
                            == 0
                    {
                        result_vector
                            [(current_block_num * register_size + byte_position) as usize] =
                            current_input_set_value;
                    };
                }
            };

            // to drop unnecessary part
            current_block = current_block & max_num;

            current_vector.push(current_block);

            if (current_block_num == exit_block) && (current_block & exit_mask > 0) {
                found = true;
                maxrow = i;
                break;
            };
        }

        prev_vector = current_vector;
        if found {
            break;
        };
    }

    println!("s = {}", input_sum);
    println!("n = {}", input_set.len());
    println!("{:?}", &result_vector);
    let mut result_numbers: Vec<u32> = Vec::new();
    if found {
        let mut start_val = 0;
        let mut jdesc: usize = input_sum as usize;
        println!("maxrow {} {}", maxrow, rows_count);
        println!("processed_rows {} {}", processed_rows, rows_count);

        for i in 0..input_set.len() {
            if jdesc <= 1 {
                break;
            };
            result_numbers.push(result_vector[jdesc] as u32);
            jdesc -= result_vector[jdesc] as usize;
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
}
