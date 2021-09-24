impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut res = String::new();
        
        let time: Vec<i32> = time.as_bytes().iter().map(|&x| (x - 48) as i32).collect();

        for (index, char) in time.iter().enumerate() {
            if index == 0 {
                if *char != 15 {
                    res += &char.to_string();
                } else {
                    if time[index + 1] != 15 && time[index + 1] > 3 {
                        res += &String::from('1');
                    } else {
                        res += &String::from('2');
                    }
                }
            }
            else if index == 1 {
                if *char != 15 {
                    res += &char.to_string();
                } else {
                    if res == String::from("2") {
                        res += &String::from('3');
                    } else {
                        res += &String::from("9");
                    }
                }
            }
            else if index == 2 {
                res += &String::from(":");
            }
            else if index == 3 {
                if *char != 15 {
                    res += &char.to_string();
                } else {
                    res += &String::from('5');
                }
            }
            else if index == 4 {
                if *char != 15 {
                    res += &char.to_string();
                } else {
                    res += &String::from('9');
                }
            }
        }
        res
    }
}
