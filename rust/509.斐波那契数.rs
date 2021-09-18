// 方法一： Vec
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut vector = vec![0, 1];
        for i in 2..=n {
            vector.push(vector[i as usize - 1] + vector[i as usize - 2]);
        }
        vector[n as usize]
    }
}

// 方法二
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => return 0,
            1 => return 1,
            _ => {
                let (mut a, mut b) = (0, 1);
                for i in 2..=n {
                    let temp = b;
                    b = a + b;
                    a = temp;
                }
                b
            }
        }
    }
}
