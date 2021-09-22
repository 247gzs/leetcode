impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut vector: Vec<u8> = Vec::new();
        let (mut x, mut y) = (num1.len(), num2.len());
        let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
        let mut carry = 0;
        while x > 0 || y > 0 {
            let a = {
                if x > 0 {
                    num1[x - 1] - '0' as u8
                } else {
                    0
                }
            };
            let b = {
                if y > 0 {
                    num2[y - 1] - '0' as u8
                } else {
                    0
                }
            };
            let d = a + b + carry;
            carry = d / 10;
            vector.push(d % 10 + '0' as u8);
            if x > 0 {x -= 1}
            if y > 0 {y -= 1}
        }
        if carry != 0 {
            vector.push(carry + '0' as u8);
        }
        vector.reverse();
        String::from_utf8(vector).unwrap()
    }
}
