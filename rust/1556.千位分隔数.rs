impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n < 100 {
            return n.to_string();
        }
        let mut d = n;
        let mut vector: Vec<String> = Vec::new();
        while d != 0 {
            let remainder = d % 1000;
            d = d / 1000;
            if d != 0 {
                vector.push(format!("{:0>3}", remainder));
            } else {
                vector.push(remainder.to_string());
            }
            vector.push(String::from('.'));
        }
        if vector[vector.len() - 1].contains(".") {
            vector.pop();
        }
        vector.reverse();
        vector.into_iter().collect::<String>()
    }
}
