-- Build once per wrk thread; send identical raw HTTP bytes to both servers.
local chunk = string.rep("x", 1024)
local framed = "400\r\n" .. chunk .. "\r\n"
local payload = string.rep(framed, 256) .. "0\r\n\r\n"
local wire = "POST /upload HTTP/1.1\r\n"
    .. "Host: localhost\r\n"
    .. "Connection: close\r\n"
    .. "transfer-encoding: chunked\r\n\r\n" .. payload

request = function()
    return wire
end

done = function(summary, latency, requests)
    io.write(string.format("completed=%d bytes=%d non_2xx_3xx=%d\n",
        summary.requests, summary.bytes, summary.errors.status))
end
