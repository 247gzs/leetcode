/**
 * @param {number[][]} matrix
 * @return {boolean}
 */
var isToeplitzMatrix = function(matrix) {
    var judge = (x, y) => {
        var num = matrix[x][y];
        while (x < m && y < n) {
            if (matrix[x][y] !== num) return false;

            x ++, y ++;
        }
        return true;
    };

    var m = matrix.length, n = matrix[0].length;

    for (var i = 0; i < m; i ++) {
        if (!judge(i, 0)) return false;
    }
    for (var i = 0; i < n; i ++) {
        if (!judge(0, i)) return false;
    }
    return true;
};
