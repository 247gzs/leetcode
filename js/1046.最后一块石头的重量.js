/**
 * @param {number[]} stones
 * @return {number}
 */
var lastStoneWeight = function(stones) {
    while (stones.length > 1) {
        stones.sort((a, b) => a-b);
        x = stones.pop();
        y = stones.pop();

        if (x == y) continue;

        stones.push(Math.abs(x - y));
    }
    return stones.length === 1 ? stones[0]: 0;
};
