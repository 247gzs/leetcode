// 方法一： String处理
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let vec: Vec<&str> = s.split(' ').collect();
        let mut s_rev: Vec<String> = Vec::new();
        for chars in vec.iter() {
            s_rev.push(chars.chars().rev().collect::<String>());
        }
        let mut string = String::new();
        for (index, chars) in s_rev.iter().enumerate() {
            if index != 0 {
                string.push(' ');
            }
            string.push_str(&chars);
        }
        string
    }
}

// 方法二： 高阶函数
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s
            .split_whitespace()
            .map(|x| x.chars().rev().collect::<String>() + " ")
            .collect::<String>()
            .trim()
            .to_string()
    }
}
