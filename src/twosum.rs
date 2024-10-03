use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = seen.get(&(target - num));
        if let Some(complement) = complement {
            return vec![*complement, i as i32];
        }
        seen.insert(*num, i as i32);
    }
    return Vec::new();
}
