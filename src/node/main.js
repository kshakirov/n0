const n0 = require("../../target/debug/n0.node");

// const input = Buffer.from([0x07, 0x41, 0xff, 0x10]);
// const output = n0.isBuffer(input);

// console.log(output);
// console.log(output === input);
// console.log(input) 

const port = 9092;
const net = require('node:net');
const server = net.createServer((c) => {
    // 'connection' listener.
    var buffer = Buffer.alloc(0);
    console.log('client connected');
    c.on('data', (d) => {
	console.log("Data from client");
	console.log(d);
	buffer = Buffer.concat([buffer,d]);
	console.log("Data at the end is ");
	console.log(buffer);
	const output = n0.isBuffer(buffer);
	console.log(buffer);
	console.log(output);
	buffer = output;
	c.write(buffer);
	c.end();

	
    })
  c.on('end', () => {
      console.log('client disconnected');
      
  });


});
server.on('error', (err) => {
  throw err;
});
server.listen(port, () => {
    console.log(`n0 Server is listening on port ${port}`);
});
