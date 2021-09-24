impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        s.chars().for_each(|c| {
            match stack.last() {
                Some(&v) if c == v => {
                    stack.pop();
                },
                _ => {
                    stack.push(c);
                }
            }
        });
        stack.iter().collect::<String>()
    }
}
