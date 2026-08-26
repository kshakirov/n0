const n0 = require("../../target/debug/n0.node");

const input = Buffer.from([0x00, 0x41, 0xff]);
const output = n0.roundtrip(input);

console.log(output);
console.log(output === input);
