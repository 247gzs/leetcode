impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // 方法一
        match haystack.find(&needle) {
            Some(idx) => idx as i32,
            None => -1,
        }
        // 方法二
        // if needle.len() == 0 {
        //     return 0;
        // }
        // let haystack = haystack.as_bytes();
        // let needle = needle.as_bytes();
        // let haystack_length = haystack.len();
        // let needle_length = needle.len();
        // for index in 0..haystack_length {
        //     if haystack[index] != needle[0] {
        //         continue;
        //     }

        //     let mut idx = index;
        //     let mut idy = 0;
        //     let mut flag = true;
        //     loop {
        //         if haystack[idx] == needle[idy] {
        //             idx += 1;
        //             idy += 1;
        //         } else {
        //             flag = false;
        //             break;
        //         }
        //         if idy == needle.len() {
        //             break;
        //         }
        //         if idx == haystack_length {
        //             flag = false;
        //             break;
        //         }
        //     }
        //     if flag == true {
        //         return index as i32;
        //     }
        // }
        // return -1 as i32;
    }
}