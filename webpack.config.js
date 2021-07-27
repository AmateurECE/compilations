const path = require('path');

module.exports = {
    mode: 'development',
    entry: './js/index.js',
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'compilations', 'static',
                           'compilations', 'js'),
    },
};
