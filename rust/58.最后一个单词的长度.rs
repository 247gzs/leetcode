pub fn length_of_last_word_by_bytes(s: String) -> i32 {
    s.bytes()
        .rev()
        .skip_while(|&a| a == b' ')
        .take_while(|&a| a != b' ')
        .count() as i32
}

pub fn length_of_last_word_by_chars(s: String) -> i32 {
    s.chars()
        .rev()
        .skip_while(|&a| a == ' ')
        .take_while(|&a| a != ' ')
        .count() as i32
}

pub fn length_of_last_word_by_vector(s: String) -> i32 {
    let vec: Vec<&str> = s.trim().rsplit(' ').collect();
    vec[0].len() as i32
}

pub fn length_of_last_word_by_string(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

pub fn length_of_last_word_by_method(s: String) -> i32 {
    let mut s: String = s;
    let mut res = 0;
    while let Some(char) = s.pop() {
        if res == 0 && char == ' ' {
            continue;
        } else if char == ' ' && res != 0 {
            break;
        } else {
            res += 1;
        }
    }
    res
}

fn main() {
    let s = String::from(" Hello World! ");
    // println!("{}", length_of_last_word_by_method(s));
    // println!("{}", length_of_last_word_by_string(s));
    // println!("{}", length_of_last_word_by_vector(s));
    // println!("{}", length_of_last_word_by_bytes(s));
    println!("{}", length_of_last_word_by_chars(s));
}

