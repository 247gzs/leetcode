impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        match word {
            string if string.to_lowercase() == word => return true,
            string if string.to_uppercase() == word => return true,
            _ => {
                // 首字母转大写
                let mut vec_char: Vec<char> = word.to_lowercase().chars().collect();
                vec_char[0] = vec_char[0].to_uppercase().nth(0).unwrap();
                // Vec<char> 转化为 String
                let new_s: String = vec_char.iter().collect();
                
                if new_s == word {
                    true
                } else {
                    false
                }
            },
        }
    }
}
