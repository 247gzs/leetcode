impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_bottles = num_bottles;
        let (mut num, mut empty_bottles) = (0i32, 0i32);
        while num_bottles != 0 || empty_bottles >= num_exchange {
            num += num_bottles;
            num += empty_bottles / num_exchange;
            let mut temp = (num_bottles + empty_bottles) / num_exchange;
            empty_bottles = (num_bottles + empty_bottles) % num_exchange;
            num_bottles = temp;
        }
        num
    }
}
