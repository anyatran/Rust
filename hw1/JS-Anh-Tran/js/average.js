// importing and initializing neccessary modules
var fs = require("fs");
var readline = require("readline");
var stream = require("stream");
var outstream = new stream;

outstream.readable = true;
outstream.writable = true;

var result = {
    sum: 0,
    l: 0,
    avg: 0,
    data: [],
    plus5: [],
    minus5: []
};

// event line
// takes a line and checks if it meets the conditions
exports.getAverage = function (someFile, callback) {
    var rl = readline.createInterface({
        input: fs.createReadStream(someFile),
        output: outstream,
        terminal: false
    });

    rl.on("line", function (line) {
        if (isNaN(parseFloat(line)) || line < 0) {

        } else {
            if (parseFloat(line) == 999) {
                rl.close();
            } else {
                update(parseFloat(line));
            }
        }

    });

    // event close
    // return the average when the program ends
    rl.on("close", function () {
        result.avg = result.sum / result.l;
        result.minus5 = result.data.filter(toAvg);
        result.plus5 = result.data.filter(fromAvg);
        console.log("AVERAGE IS :" + result.avg);
        console.log("TEMPERATURES WITHIN [" + Number(parseFloat(result.avg) - 5) + ", " + result.avg + "]: " + result.minus5);
        console.log("TEMPERATURES WITHIN [" + result.avg + ", " + Number(parseFloat(result.avg) + 5) + "]: " + result.plus5);
        callback(result);
        process.exit(0);
    });
};

// increment the sum by the temperature from stdin
// increment length by 1
// and add the temp to data
function update(temp) {
    result.sum += parseFloat(temp);
    result.l++;
    result.data[result.data.length] = temp;
};

// checks if the number is within [avg-5, avg]
function toAvg(temp) {
    return (temp <= result.avg && temp >= (result.avg - 5));
};

// checks if the number is within [avg, avg+5]
function fromAvg(temp) {
    return (temp >= result.avg && temp <= (result.avg + 5));
};
