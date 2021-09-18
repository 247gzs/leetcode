impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let length = flowerbed.len();
        if length == 1 {
            if n == 1 && flowerbed[0] == 0 || n == 0 {
                return true;
            } else {
                return false;
            }
        }

        let (mut n, mut flowerbed) = (n, flowerbed);
        for i in 0..length {
            if i == 0 && flowerbed[i + 1] == 0 && flowerbed[i] == 0 {
                n -= 1;
                flowerbed[i] = 1;
            } else if i == length - 1 && flowerbed[i - 1] == 0 && flowerbed[length - 1] == 0 {
                n -= 1;
                flowerbed[i] = 1;
            } else if i != 0
                && i != length - 1
                && flowerbed[i - 1] == 0
                && flowerbed[i + 1] == 0
                && flowerbed[i] == 0
            {
                n -= 1;
                flowerbed[i] = 1;
            } else {
                continue;
            }
        }
        n <= 0
    }
}
