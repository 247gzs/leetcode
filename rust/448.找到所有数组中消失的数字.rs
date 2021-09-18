// 方法一： HashMap
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let n = nums.len();
        let mut map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            map.insert(num, index);
        }
        let mut vec = Vec::new();
        for num in 1..=n {
            let num = num as i32;
            match map.get(&num) {
                Some(i) => continue,
                None => vec.push(num),
            }
        }
        vec
    }
}

// 方法二： HashSet
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let n = nums.len();
        let mut set = HashSet::new();
        let mut vec = Vec::new();
        for num in nums.iter() {
            set.insert(*num);
        }
        for num in 1..=n {
            let num = num as i32;
            if !set.contains(&num) {
                vec.push(num);
            }
        }
        vec
    }
}

// 方法三：Vec
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut vector = vec![false; n];
        for num in nums {
            vector[num as usize - 1] = true;
        }
        
        let mut result = vec![];
        for (index, flag) in vector.iter().enumerate() {
            if !flag {
                result.push((index + 1) as i32);
            }
        }
        result
    }
}

// 方法四：高阶函数
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums
            .iter()
            .enumerate()
            .map(|(idx, _)| idx as i32 + 1)
            .collect::<Vec<i32>>();
        nums.iter()
            .enumerate()
            .for_each(|(_, &num)| result[num as usize - 1] = 0);
        result
            .into_iter()
            .filter(|&num| num > 0)
            .collect::<Vec<i32>>()
    }
}
