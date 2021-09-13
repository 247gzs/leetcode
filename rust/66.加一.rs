impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry: i32 = 1;
        let mut vector = Vec::new();
        let mut new_digits = digits.to_owned();
        new_digits.reverse();
        for digit in new_digits {
            vector.push((carry + digit) % 10);
            carry = (carry + digit) / 10;
        }
        if carry != 0 {
            vector.push(carry);
        }
        vector.reverse();
        vector
    }

    /*
    // 方法二： map
    let mut bit = 1;
    let mut digits = digits.to_owned();
    digits.reverse();
    digits = digits.iter().map(|digit| {
        let data = bit + digit;
        bit = data / 10;
        data % 10
    }).collect();
    if bit != 0 {
        digits.push(bit);
    }
    digits.reverse();
    digits
    */
}
