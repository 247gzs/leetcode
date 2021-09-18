// 方法一
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums;
        nums.sort_unstable_by(|x, y| y.cmp(x));

        let mut sum = 0;
        for (idx, num) in nums.iter().enumerate() {
            if idx % 2 == 0 {
                continue;
            }
            sum += num;
        }
        sum
    }
}

// 方法二： 高阶函数
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums;
        nums.sort_unstable();
        nums.into_iter().step_by(2).sum()
    }
}
