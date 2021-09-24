impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>() - salary.iter().min().unwrap() - salary.iter().max().unwrap())
            as f64
            / (salary.len() as i32 - 2) as f64
    }
}
