/**
 * @param {string} s
 * @param {string} t
 * @return {character}
 */
var findTheDifference = function(s, t) {
    const cnt = new Array(26).fill(0), chCode = 'a'.charCodeAt();
    // 统计每个单词出现的次数
    for (let ch of s) {
        cnt[ch.charCodeAt() - chCode] ++;
    }
    for (let ch of t) {
        cnt[ch.charCodeAt() - chCode] --;
        if (cnt[ch.charCodeAt() - chCode] < 0) {
            return ch;
        }
    }
    return ' ';
};
