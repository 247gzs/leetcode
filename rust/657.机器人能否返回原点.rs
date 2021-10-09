impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut x, mut y) = (0, 0);
        for char in moves.chars().into_iter() {
            if char == 'U' {
                y += 1;
            } else if char == 'D' {
                y -= 1;
            } else if char == 'L' {
                x -= 1;
            } else {
                x += 1;
            }
        }
        if x == 0 && y == 0 {
            return true;
        } else {
            return false;
        }
    }
}
