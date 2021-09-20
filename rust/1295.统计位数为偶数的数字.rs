impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        fn find_digit(num: i32) -> bool {
            let (mut num, mut digit) = (num, 0);
            while num != 0 {
                num /= 10;
                digit += 1;
            }
            if digit % 2 == 0 {
                true
            } else {
                false
            }
        }

        let mut res: i32 = 0;
        for num in nums.into_iter() {
            if find_digit(num) {
                res += 1;
            }
        }
        res
    }
}
