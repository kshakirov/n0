
const net = require('node:net');
const port = 9092;
const client = net.createConnection({ port: port }, () => {
  // 'connect' listener.
  console.log('connected to server!');
  client.write(Buffer.from([0x07, 0x41, 0xff, 0x10]));
});
client.on('data', (data) => {
  console.log(data);
  client.end();
});
client.on('end', () => {
  console.log('disconnected from server');
});
