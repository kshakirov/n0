const n0 = require("../../target/debug/n0.node");

const input = Buffer.from([0x07, 0x41, 0xff]);
const output = n0.isBuffer(input);

console.log(output);
console.log(output === input);
console.log(input)
