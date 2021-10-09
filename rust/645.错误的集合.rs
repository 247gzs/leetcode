impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let len = nums.len() as i32;
        let mut sum = 0;
        let mut set = HashSet::new();
        let mut redundant = 0;
        for num in nums {
            if set.contains(&num) == true {
                redundant = num;
            }
            sum += num;
            set.insert(num);
        }
        let missing = (len * len + len) / 2 - sum + redundant;
        vec![redundant, missing]
    }
}
