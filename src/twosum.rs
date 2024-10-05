use std::collections::HashMap;
// for leetcode
pub fn two_sum_vec_res(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

pub fn two_sum_tuple_res(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = seen.get(&(target - num));
        if let Some(complement) = complement {
            return Some((*complement as usize, i));
        }
        seen.insert(*num, i as i32);
    }
    return None;
}

pub fn test_two_sum() {
    assert_eq!(two_sum_vec_res(vec![1, 2, 3, 4, 5], 6), vec![1, 3]);
    assert_eq!(two_sum_tuple_res(vec![1, 2, 3, 4, 5], 6), Some((1, 3)));
    assert_eq!(two_sum_tuple_res(vec![1, 2, 2, 2, 2], 6), None);
}
