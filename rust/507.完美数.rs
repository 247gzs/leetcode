impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let n = (num as f64).sqrt() as i32;
        let mut sum = 1i32;
        for i in 2..=n {
            if num % i == 0 {
                sum += i + num / i;
            }
        } 
        sum == num
    }
}
