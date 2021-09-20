impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut vector: Vec<i32> = Vec::new();
        let mut n = n;
        if n % 2 != 0 {
            vector.push(0);
        }
        n -= 1;
        let (mut x, mut y) = (1, -1);
        while n > 0 {
            vector.push(x);
            vector.push(y);
            x += 1;
            y -= 1;
            n -= 2;
        }
        vector
    }
}
