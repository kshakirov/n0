local payload = string.rep("x", 256 * 1024)
local wire = "POST /upload HTTP/1.1\r\nHost: localhost\r\n"
    .. "Connection: close\r\ncontent-length: 262144\r\n\r\n" .. payload

request = function() return wire end

done = function(summary)
    io.write(string.format("completed=%d bytes=%d non_2xx_3xx=%d\n",
        summary.requests, summary.bytes, summary.errors.status))
end
