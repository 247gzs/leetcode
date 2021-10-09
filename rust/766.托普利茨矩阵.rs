impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        for index in 0..m {
            if Solution::judge(index, 0, m, n, &matrix) == false {
                return false;
            }
        }
        for index in 0..n {
            if Solution::judge(0, index, m, n, &matrix) == false {
                return false;
            }
        }
        true
    }

    pub fn judge(x: i32, y: i32, m: i32, n: i32, matrix: &Vec<Vec<i32>>) -> bool {
        let num = matrix[x as usize][y as usize];
        let (mut x, mut y) = (x, y);
        while x < m && y < n {
            if matrix[x as usize][y as usize] == num {
                x += 1;
                y += 1;
            } else {
                return false;
            }
        }
        true
    }
}
