impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut vector: Vec<i32> = arr.clone();
        vector.sort();
        let mut rank: i32 = 1;
        let mut map = HashMap::new();
        for (index, data) in vector.iter().enumerate() {
            if index == 0 || (index > 0 && vector[index] == vector[index - 1]) {
                map.insert(data, rank);
                continue;
            } else {
                rank += 1;
                map.insert(data, rank);
            }
        }
        
        let mut result: Vec<i32> = vec![];
        for data in arr {
            result.push(*map.get(&data).unwrap())
        }
        result
    }
}
