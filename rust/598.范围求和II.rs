impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let min_x = ops.iter().map(|op| op[0]).min().unwrap_or(m);
        let min_y = ops.iter().map(|op| op[1]).min().unwrap_or(n);
        min_x * min_y
    }
}
