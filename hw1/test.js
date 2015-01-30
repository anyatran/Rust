var readline = require("readline");
var fs = require("fs");

var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

var sum = 0;
var l = 0;
var avg;
var buffer, bufferString, bufferSplit;

function readData(callback) {
    fs.readFile(process.argv[2], function(error, data) {
        if (error) throw error;
        buffer = data;
        bufferString = buffer.toString();
        bufferSplit = bufferString.split("\n");
        callback();
    });
};

function average() {
    for(var i = 0; i < bufferSplit.length; i++) {
        if (buffer[i] !== 999) {
            sum += buffer[i];
            l++;
        }
        else {
            avg =  sum / l;
        }
        //console.log(parseFloat(bufferSplit[i]) + 3);
    };
    console.log(avg);
};
readData(average);
/*
var rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.question("what's your name?", function(answer) {
    console.log("merci", answer);

    rl.close();
}); */

