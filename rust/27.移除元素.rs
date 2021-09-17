impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // 方法一：使用内置的方法
        // nums.retain(|&x| x != val);
        // nums.len() as i32

        // 方法二
        let mut index = 0;
        for idx in 0..nums.len() {
            if nums[idx] != val {
                nums[index] = nums[idx];
                index += 1;
            }
        }
        index as i32
    }
}
