impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut flag = false;
        for index in 2..arr.len() {
            if arr[index] % 2 != 0 && arr[index - 1] % 2 != 0 && arr[index - 2] % 2 != 0 {
                flag = true;
                break;
            }
        }
        flag
    }
}
