impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn existed_zero(number: i32) -> bool {
            let mut number:i32 = number;
            while number != 0 {
                if number % 10 == 0 {
                    return false;
                }
                number /= 10;
            }
            true
        }

        for i in 1..(n / 2 + 1) {
            if existed_zero(i as i32) && existed_zero(n - i as i32) {
                return vec![i as i32, (n - i) as i32];
            }
        }
        vec![]
    }
}
