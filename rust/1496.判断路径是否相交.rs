impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        use std::collections::HashSet;

        let (mut x, mut y) = (0, 0);
        let mut hs = HashSet::new();
        hs.insert((x, y));
        for char in path.chars().into_iter() {
            if char == 'N' {
                y += 1;
            }
            else if char == 'S' {
                y -= 1;
            }
            else if char == 'E' {
                x += 1;
            }
            else {
                x -= 1;
            }
            if hs.contains(&(x, y)) {
                return true;
            }
            else {
                hs.insert((x, y));
            }
        }
        false
    }
}
