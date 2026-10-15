use n0_core::{
    parse_http_body, parse_http_header, parse_http_req, BodyParserState, BodyParserStatus,
    NetworkInput,
};
fn main() {
    let _s1   = "GET /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\n\r\n";
    let _s = "GET /users/123 HTTP/1.1\r\n\r\n";

    let s = "POST /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\ncontent-length: 10\r\n\r\n0123456789";

    let mut buf: Vec<u8> = s.as_bytes().to_vec();
    let mut header_state: Vec<i32> = [0, 1, 0, 4].to_vec();
    let mut parser_state: Vec<i32> = [0, 0, 0, 0, 0].to_vec();
    let r = parse_http_header(&mut buf, &mut header_state);
    assert!(header_state[0] == 3);
    assert!(header_state[1] == 1);
    assert!(header_state[2] == 10);
    println!("counter is {} at the position  is {}\n", r, buf[r]);
    assert!(buf[r] == 48);

    let mut arena = vec![0u8; 10];
    let mut state = BodyParserState {
        state: BodyParserStatus::ReadChunkData,
        networking_input: NetworkInput::ReadingFixedData,
        current_value: header_state[2] as usize,
        arena: &mut arena,
        arena_ptr: 0,
    };
    parse_http_body(&mut state, &buf, r);
    parse_http_req(&buf, &mut arena, &mut header_state, &mut parser_state);
}
