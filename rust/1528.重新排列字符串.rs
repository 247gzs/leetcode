impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for (index, char) in s.chars().enumerate() {
            map.insert(indices[index] as i32, char.to_string());
        }
        let mut res = String::new();
        for index in 0..s.len() {
            res += map.get(&(index as i32)).unwrap();
        }
        res
    }
}
