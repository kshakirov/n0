
const net = require('node:net');
const port = 9092;
const client = net.createConnection({ port: port }, () => {
  // 'connect' listener.
    console.log('connected to server!');

    const raw_get_request = "POST /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\ncontent-length: 10\r\n\r\n0123456789";
    client.write(Buffer.from(Buffer.from(raw_get_request)));
});
client.on('data', (data) => {
  console.log(data);
  client.end();
});
client.on('end', () => {
  console.log('disconnected from server');
});
