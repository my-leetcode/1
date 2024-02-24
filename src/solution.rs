use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let tmp = nums.clone();
        let map = tmp
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i))
            .collect::<HashMap<_, _>>();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("no solution found")
    }
}

pub struct Solution;
