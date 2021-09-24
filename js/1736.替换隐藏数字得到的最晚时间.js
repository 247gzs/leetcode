/**
 * @param {string} time
 * @return {string}
 */
var maximumTime = function(time) {
    time = time.split('');
    var mapping = {
        0: '2',
        1: '3',
        3: '5',
        4: '9'
    }
    for (var i = 0; i < time.length; i ++) {
        if (time[i] === '?') {
            time[i] = mapping[i];
            if (i === 1 && time[i - 1] !== '2') {
                time[i] = '9';
            }
            if (i === 0 && parseInt(time[i + 1]) > 3) {
                time[i] = '1';
            }
        }
    }
    return time.join('');
};

