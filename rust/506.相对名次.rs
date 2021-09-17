impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        use std::collections::HashMap;
        let mut orders = score.clone();
        orders.sort_unstable_by(|x, y| y.cmp(x));

        let mut orders = orders
            .into_iter()
            .enumerate()
            .map(|(idx, order)| (order, idx + 1))
            .collect::<HashMap<i32, usize>>();

        score
            .into_iter()
            .map(|sco| match orders.get(&sco).unwrap() {
                1 => String::from("Gold Medal"),
                2 => String::from("Silver Medal"),
                3 => String::from("Bronze Medal"),
                _ => orders.get(&sco).unwrap().to_string(),
            })
            .collect::<Vec<String>>()
    }
}
