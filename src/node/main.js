const n0 = require("../../target/debug/n0.node");

// const input = Buffer.from([0x07, 0x41, 0xff, 0x10]);
// const output = n0.isBuffer(input);

// console.log(output);
// console.log(output === input);
// console.log(input) 

const port = 9092;
const HEADER = 0;
const FINISHED = 2;
const ARENA_PTR = 3;
const net = require('node:net');
const server = net.createServer((c) => {
    // 'connection' listener.
    var buffer = Buffer.alloc(0);
    const headerData = new Int32Array([0 , 0, 0, 0]),
	  parserState = new Int32Array([0,0,0,0,0]),
          // Статус тела, сетевой режим, current_value, arena_ptr.
          bodyState = new Int32Array([0,0,0,0]);
    console.log('client connected');
    c.on('data', (d) => {
	console.log("Data from client");
	console.log(d);
	// До конца заголовков сохраняем вход; тело передаём порциями.
	let input = d;
	if (parserState[0] === HEADER) {
	    buffer = Buffer.concat([buffer, d]);
	    input = buffer;
	}
	//here on purpose we won't keep it or accumulate it must be handled by users 
	let arena = Buffer.alloc(d.length);
	n0.n_parse_http_req(input, arena, headerData, parserState, bodyState);
	if (parserState[0] !== HEADER) buffer = Buffer.alloc(0);
	console.log(headerData);
	console.log(bodyState);
	const bodyPart = arena.subarray(0, bodyState[ARENA_PTR]);
	console.log(bodyPart);
	if (bodyPart.length > 0) c.write(bodyPart);
	if (parserState[0] === FINISHED) c.end();

	
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
