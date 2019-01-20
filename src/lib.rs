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
                new_vector[column_index] =
                    prev_vector[column_index] || prev_vector[column_index - previous_row_cell_value];
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


pub fn find_subset(input_sum: &u32, input_set: &Vec<u32>) -> Vec<u32> {
    let mut result_numbers: Vec<u32> = Vec::new();
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

}
