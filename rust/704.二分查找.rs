impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = ((left + right) / 2) as usize;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                // 边界控制，无符号整型，若0-1，会返回一个异常大的数
                if mid == 0 {
                    return -1;
                }
                right = mid - 1;
            }
        }
        -1
    }
}
