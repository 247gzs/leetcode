impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for n in left..right + 1 {
            if Solution::is_valid(n) == true {
                result.push(n);
            }
        }
        return result;
    }

    fn is_valid(n: i32) -> bool {
        let mut number = n;
        while number != 0 {
            let remainder = number % 10;
            if remainder == 0 || n % remainder != 0 {
                return false;
            }
            number = number / 10;
        }
        true
    }
}

// 方法二：
// impl Solution {
//     pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
//         (left..right+1).filter(|&x| Solution::is_valid(x)).collect()
//     }
