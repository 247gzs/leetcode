impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let dy1: Vec<i32> = date1
            .split("-")
            .map(|date| date.parse::<i32>().unwrap())
            .collect();
        let dy2: Vec<i32> = date2
            .split("-")
            .map(|date| date.parse::<i32>().unwrap())
            .collect();
        (Solution::diff_day(dy1[0], dy1[1], dy1[2]) - Solution::diff_day(dy2[0], dy2[1], dy2[2])).abs()
    }

    pub fn diff_day(year: i32, month: i32, day: i32) -> i32 {
        let mut y = year;
        let mut m = month;
        let d = day;
        if m < 3 {
            y -= 1;
            m += 12;
        }
        (y - 1) * 365 + y / 4 - y / 100 + y / 400 + (m - 1) * 28 + 13 * (m + 1) / 5 - 7 + d
    }
}
