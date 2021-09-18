impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::cmp::min;
        use std::collections::HashMap;

        // 记录每个字符串所在的索引
        let map = list1
            .into_iter()
            .enumerate()
            .map(|(idx, string)| (string, idx))
            .collect::<HashMap<String, usize>>();

        let mut min_index = 2001i32;
        // 获取最小的索引
        for (idx, string) in list2.iter().enumerate() {
            // is_some 用于判断是否存在结果
            if map.get(string).is_some() {
                min_index = min(idx as i32 + *(map.get(string).unwrap()) as i32, min_index);
            }
        }

        let mut vector: Vec<String> = Vec::new();
        for (idx, string) in list2.iter().enumerate() {
            if map.get(string).is_some() {
                if idx as i32 + *(map.get(string).unwrap()) as i32 == min_index {
                    vector.push(string.to_string());
                }
            }
        }
        vector
    }
}
