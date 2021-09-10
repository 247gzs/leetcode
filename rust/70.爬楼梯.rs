impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut prev = 2;
        let mut prev_prev = 1;
        let mut index = 3;
        while index <= n {
            let next = prev + prev_prev;
            prev_prev = prev;
            prev = next;
            index += 1;
        }
        prev
    }
}
