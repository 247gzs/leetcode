impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        // 统计每个字符出现的次数
        for char in chars.chars().into_iter() {
            if map.get(&char).is_none() {
                map.insert(char, 1);
            } else {
                map.insert(char, map.get(&char).unwrap() + 1);
            }
        }

        let mut num = 0i32;
        for string in words.iter() {
            // clone，后续处理需要改变map集合，而map集合适用于所有的字符串
            let mut map_iter = map.clone();
            // 标记次数
            let mut flag = true;
            let s_len = string.len() as i32;
            num += s_len;
            for char in string.chars().into_iter() {
                if map_iter.get(&char).is_none() {
                    flag = false;
                    break;
                }
                // 如果单词存在，存在的次数减1；减至0时，从map中移除该元素
                let count = map_iter.get(&char).unwrap();
                if *count == 1 {
                    map_iter.remove(&char);
                } else {
                    map_iter.insert(char, count - 1);
                }
            }
            if flag != true {
                num -= s_len;
            }
        }
        num
    }
}
