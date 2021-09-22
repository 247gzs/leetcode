impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut map: HashMap<i32, ()> = HashMap::new();
        for data in arr.into_iter() {
            if let Some(_) = map.get(&(data * 2)) {
                return true;
            }

            if data % 2 == 0 {
                if let Some(_) = map.get(&(data / 2)) {
                    return true;
                }
            }
            map.insert(data, ());
        }
        false
    }
}
