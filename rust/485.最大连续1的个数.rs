impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut start, mut end, mut max_len) = (0i32, 0i32, 0i32);
        for (index, num) in nums.iter().enumerate() {
            if *num == 0 {
                if end - start > max_len {
                    max_len = end - start;
                }
                start = index as i32;
                end = index as i32;
            } else {
                end += 1;
            }
        }
        if end - start > max_len {
            max_len = end - start;
        }
        max_len
    }
}
