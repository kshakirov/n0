use n0_core::parse_http_header;
fn main() {
    let _s1   = "GET /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\n\r\n";
    let _s = "GET /users/123 HTTP/1.1\r\n\r\n";

    let s = "POST /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\ncontent-length: 10\r\n\r\n0123456789";

    let mut buf: Vec<u8> = s.as_bytes().to_vec();
    let mut vec: Vec<i32> = [0, 1, 0, 4].to_vec();
    let _r = parse_http_header(&mut buf, &mut vec);
    assert!(vec[0] == 3);
    assert!(vec[1] == 1);
    assert!(vec[2] == 10);
}
