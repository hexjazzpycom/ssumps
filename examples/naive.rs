extern crate ssumps;

use ssumps::find_subset_naive;

fn main() {
    let input_set = vec![2, 4, 3, 4, 5, 5, 6, 7, 8, 11, 13, 11, 14];
    let input_sum = 55u32;
    println!("Input set: {:?}", input_set);
    println!("Input sum: {}", input_sum);
    println!("Complexity: {}", input_sum * (input_set.len() as u32));
    let output_set = find_subset_naive(&input_sum, &input_set);
    println!("Result: {:?}", output_set);
}
