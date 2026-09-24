use n0_core::parse_http_header;
fn main() {
    let s1   = "GET /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\n\r\n";
    let s = "GET /users/123 HTTP/1.1\r\n\r\n";

    let mut buf: Vec<u8> = s.as_bytes().to_vec();
    let mut vec: Vec<i32> = [0, 1, 0, 4].to_vec();
    let r = parse_http_header(&mut buf, &mut vec);
}
