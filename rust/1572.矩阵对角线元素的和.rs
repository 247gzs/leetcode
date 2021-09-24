impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat.len();
        let (mut x, mut y) = (0, 0);
        let mut sum = 0;
        while x < len {
            sum += mat[x][y];
            x += 1;
            y += 1;
        }

        let (mut x, mut y) = (0, len - 1);
        while x < len {
            sum += mat[x][y];
            x += 1;
            y -= 1;
        }

        if len % 2 != 0 {
            sum -= mat[len / 2][len / 2];
        }
        sum
    }
}
