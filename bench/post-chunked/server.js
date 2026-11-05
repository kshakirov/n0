"use strict";

const net = require("node:net");
const http = require("node:http");
const path = require("node:path");
const variant = process.env.VARIANT;
const expectedBytes = 256 * 1024;
const port = Number(process.env.PORT || 18083);
const host = "127.0.0.1";
const ok = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
const bad = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
let server;

if (variant === "n0") {
    const addon = require(path.resolve(__dirname, "../../target/release/n0.node"));
    server = net.createServer(socket => {
        const headers = new Int32Array(4);
        const parser = new Int32Array(5);
        const body = new Int32Array(5);
        let pendingHeader = Buffer.alloc(0);
        let received = 0;
        let done = false;
        socket.setNoDelay(true);
        socket.on("error", () => socket.destroy());
        socket.on("data", data => {
            if (done) return;
            let input = data;
            if (parser[0] === 0) {
                pendingHeader = Buffer.concat([pendingHeader, data]);
                input = pendingHeader;
            }
            const arena = Buffer.alloc(input.length);
            addon.n_parse_http_req(input, arena, headers, parser, body);
            if (parser[0] !== 0) pendingHeader = Buffer.alloc(0);
            // Handler consumes each portion immediately, retaining no body data.
            received += body[3];
            if (body[0] === 5) {
                done = true;
                socket.end(bad);
            } else if (parser[0] === 2) {
                done = true;
                socket.end(headers[0] === 3 && received === expectedBytes ? ok : bad);
            }
        });
    });
} else if (variant === "node-http") {
    server = http.createServer((request, response) => {
        let received = 0;
        request.on("data", data => { received += data.length; });
        request.on("end", () => {
            response.sendDate = false;
            response.setHeader("Connection", "close");
            const valid = request.method === "POST" && received === expectedBytes;
            response.statusCode = valid ? 200 : 400;
            response.setHeader("Content-Length", valid ? "2" : "0");
            response.end(valid ? "OK" : undefined);
        });
        request.on("error", () => response.destroy());
    });
    server.on("connection", socket => socket.setNoDelay(true));
} else {
    throw new Error("Set VARIANT=n0 or VARIANT=node-http");
}

process.once("SIGINT", () => server.close());
process.once("SIGTERM", () => server.close());
server.listen(port, host, () => console.log(`${variant}: ${host}:${port}`));
