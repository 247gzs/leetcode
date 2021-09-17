impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - *num)) {
                Some(&idx) => return vec![idx, index as i32],
                None => map.insert(*num, index as i32),
            };
        }
        vec![]
    }
}
