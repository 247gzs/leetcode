impl Solution {
    pub fn check_record(s: String) -> bool {
        if s.chars().filter(|&c| c == 'A').count() < 2 && !s.contains("LLL") {
            return true;
        }
        false
    }
}
