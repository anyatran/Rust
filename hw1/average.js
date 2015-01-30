var fs = require("fs");
var readline = require("readline");

var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

var buf = new Buffer();
var sum = 0;
var l = 0;
process.stdin.on("readable", function() {
    var chunk = process.stdin.read();
    if (chunk !== null) {
        var n = chunk.readDouleLE(0);
        if (n === 999) {
            // done
        };
        if (n > 0) {
            sum += n;
            l++;
        };
    };
});
