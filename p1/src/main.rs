fn main() {
    two_sum(vec![2,7,11,15], 9);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let count = nums.len();
    let mut number_index_hash_map = std::collections::HashMap::with_capacity(count);
    for (value, index) in nums.into_iter().zip(0..) {
        let expected_number = target - value;
        match number_index_hash_map.get(&expected_number) {
            None => {
                number_index_hash_map.insert(value, index);
            }
            Some(&previous_index) => return vec![previous_index, index],
        }
    }
    vec![]
}
