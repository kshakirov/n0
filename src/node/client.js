
const net = require('node:net');
const port = 9092;
const client = net.createConnection({ port: port }, () => {
  // 'connect' listener.
    console.log('connected to server!');

    const raw_get_request_1 = "POST /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\ncontent-length: 10\r\n\r\n0123";
    client.write(Buffer.from(Buffer.from(raw_get_request_1)));

});
client.on('data', (data) => {
    console.log(data);
    const raw_get_request_2 = "456789";

    if(data.length >= 6){
	console.log("Must be the second response");
	client.end();
    }else{
	console.log("the first response");
	client.write(Buffer.from(Buffer.from(raw_get_request_2)));
    }
});
client.on('end', () => {
  console.log('disconnected from server');
});
