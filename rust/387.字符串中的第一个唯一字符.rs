impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        // 统计单词数量
        // 方法一
        s.chars().for_each(|char| *map.entry(char).or_insert(0) += 1);
        // 方法二
        for char in s.chars().into_iter() {
            if map.get(&char).is_none() {
                map.insert(char, 1);
            } else {
                map.insert(char, map.get(&char).unwrap() + 1);
            }
        }
        
        for (index, char) in s.chars().into_iter().enumerate() {
            if *map.get(&char).unwrap() == 1 {
                return index as i32;
            }
        }
        return -1;
    }
}
