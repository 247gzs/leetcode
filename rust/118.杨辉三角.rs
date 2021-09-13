impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut vector = Vec::new();
        for i in 0..num_rows as usize {
            match i {
                0 => vector.push(vec![1]),
                1 => vector.push(vec![1; 2]),
                n => {
                    let mut item = Vec::new();
                    {
                        item.push(1);
                        let pre_item = &vector[n - 1];
                        for j in 1..n {
                            item.push(pre_item[j - 1] + pre_item[j]);
                        }
                        item.push(1);
                    }
                    vector.push(item);
                }
            }
        }
        vector
    }
}
