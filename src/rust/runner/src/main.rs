use n0_core::{
    parse_http_body, parse_http_header, parse_http_req, BodyParserState, BodyParserStatus,
    HeaderParserState, NetworkInput,
};
fn main() {
    let _s1   = "GET /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\n\r\n";
    let _s = "GET /users/123 HTTP/1.1\r\n\r\n";

    let s = "POST /users/123 HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: SclerotixTest\r\nAccept: */*\r\ncontent-length: 10\r\n\r\n0123456789";

    let mut buf: Vec<u8> = s.as_bytes().to_vec();
    let mut header_state: Vec<i32> = [0, 1, 0, 4].to_vec();
    let mut parser_state: Vec<i32> = [0, 0, 0, 0, 0].to_vec();
    let (r, state) = parse_http_header(&mut buf, &mut header_state);
    assert!(state == HeaderParserState::Success);
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
    parse_http_req(&buf, &mut header_state, &mut parser_state, &mut state);

    // Один запрос в трёх порциях, состояние общее, арена каждый раз новая.
    let parts: [&[u8]; 3] = [
        b"POST /users/123 HTTP/1.1\r\ncontent-length: 10\r\n\r\n0123",
        b"456",
        b"789",
    ];
    let _expected_parts: [&[u8]; 3] = [b"0123", b"456", b"789"];
    let _expected_remaining = [6, 3, 0];
    let mut header_state = [0i32; 4];
    let mut parser_state = [0i32; 5];
    let mut chunked_state = BodyParserState {
        state: BodyParserStatus::ReadChunkData,
        networking_input: NetworkInput::ReadingFixedData,
        current_value: header_state[2] as usize,
        arena: &mut arena,
        arena_ptr: 0,
    };
    parse_http_req(
        parts[0],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    println!("Порция {}: состояние {:?}", 0, chunked_state);
    parse_http_req(
        parts[1],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    println!("Порция {}: состояние {:?}", 1, chunked_state);
    parse_http_req(
        parts[2],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    assert!(parser_state[0] == 2);
    println!("Порция {}: состояние {:?}", 2, chunked_state);
}
