var fs = require("fs");
var readline = require("readline");

var pathToFile = process.argv[2];

var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.on("line", function(line) {
    console.log(line);
});
