// 方法一
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        use std::collections::HashMap;

        let (mut map1, mut map2) = (HashMap::new(), HashMap::new());
        s.chars().for_each(|char| *map1.entry(char).or_insert(0) += 1);
        t.chars().for_each(|char| *map2.entry(char).or_insert(0) += 1);

        for (char, num) in map2.iter() {
            if map1.get(char).is_none() || *map1.get(char).unwrap() + 1 == *num {
                return *char;
            }
        }
        return ' ';
    }
}

// 方法二
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s
        .as_bytes()
        .iter()
        .chain(t.as_bytes().iter())
        .fold(0, |x, y| x ^ y) as char
    }
}
