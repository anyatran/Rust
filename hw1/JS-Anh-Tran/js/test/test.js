// Used Mochajs to test the progr

var assert = require("assert"); // node.js core module
var average = require("../average.js");
var path = require("path");

var path1 = path.join(__dirname, '/../../file.txt');

var expectedResult1 = {
    sum: 95.29,
    l: 5,
    avg: 19.058,
    data: [10, 20, 11.56, 23.73, 30],
    plus5: [20, 23.73],
    minus5: []
};
describe('Array', function () {
    describe('#indexOf()', function () {
        it('should return -1 when the value is not present', function () {
            assert.equal(-1, [1, 2, 3].indexOf(4)); // 4 is not present in this array so indexOf returns -1
        })
    })
});

describe('Average', function () {
    describe('#getAverage()', function () {
        it('{sum: 95.29,l: 5, avg: 19.058, data: [ 10, 20, 11.56, 23.73, 30 ], plus5: [ 20,23.73 ], minus5: []}', function (done) {
            average.getAverage(path1, function (r) {
                assert.equal(JSON.stringify(expectedResult1), JSON.stringify(r));
                done();
            })
        })
    })
});


