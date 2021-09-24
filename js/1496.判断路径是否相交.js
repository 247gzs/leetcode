/**
 * @param {string} path
 * @return {boolean}
 */
var isPathCrossing = function(path) {
    var mapping = {}, x = 0, y = 0;
    mapping[x.toString() + y.toString()] = true;
    for (var i = 0; i < path.length; i ++) {
        if (path[i] === 'N') y ++;
        else if (path[i] === 'S') y --;
        else if (path[i] === 'W') x --;
        else x ++;

        let str = x.toString() + y.toString();
        if (mapping[str] === undefined) mapping[str] = true;
        else return true; 
    }
    
    return false;
};
