impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vector = vec![];
        for char in s.chars() {
            if vector.len() == 0 {
                vector.push(char);
                continue;
            }
            if vector[vector.len() - 1] == '(' && char == ')' {
                vector.pop();
            } else if vector[vector.len() - 1] == '{' && char == '}' {
                vector.pop();
            } else if vector[vector.len() - 1] == '[' && char == ']' {
                vector.pop();
            } else {
                vector.push(char);
            }
        }
        if vector.len() != 0 {
            return false;
        }
        true
    }
}
