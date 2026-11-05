use n0_core::{
    parse_http_body, parse_http_header, parse_http_req, BodyParserState, BodyParserStatus,
    ContentType, DelimiterContext, HeaderParserState, Method, NetworkInput,
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
    assert!(header_state[0] == Method::POST as i32);
    assert!(header_state[1] == ContentType::ContentLength as i32);
    assert!(header_state[2] == 10);
    assert!(buf[r] == 48);

    let mut arena = vec![0u8; 10];
    let mut state = BodyParserState {
        state: BodyParserStatus::ReadFixedData,
        networking_input: NetworkInput::ReadingFixedData,
        current_value: header_state[2] as usize,
        arena: &mut arena,
        arena_ptr: 0,
        delimiter_context: DelimiterContext::NoDelimiter,
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
        state: BodyParserStatus::ReadFixedData,
        networking_input: NetworkInput::ReadingFixedData,
        current_value: header_state[2] as usize,
        arena: &mut arena,
        arena_ptr: 0,
        delimiter_context: DelimiterContext::NoDelimiter,
    };
    parse_http_req(
        parts[0],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    assert_eq!(&chunked_state.arena[..chunked_state.arena_ptr], b"0123");
    let mut second_arena = [0u8; 3];
    chunked_state.arena = &mut second_arena;
    parse_http_req(
        parts[1],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    assert_eq!(&chunked_state.arena[..chunked_state.arena_ptr], b"456");
    let mut third_arena = [0u8; 3];
    chunked_state.arena = &mut third_arena;
    parse_http_req(
        parts[2],
        &mut header_state,
        &mut parser_state,
        &mut chunked_state,
    );
    assert!(parser_state[0] == 2);
    assert_eq!(&chunked_state.arena[..chunked_state.arena_ptr], b"789");

    // Один чанк с телом abc и завершающий нулевой чанк.
    let chunk = b"3\r\nabc\r\n0\r\n\r\n";
    let request = b"POST /users/123 HTTP/1.1\r\nHost: localhost:9092\r\ntransfer-encoding: chunked\r\n\r\n3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n";

    let (_, state) = parse_http_header(request, &mut header_state);
    assert!(state == HeaderParserState::Success);
    assert!(header_state[0] == Method::POST as i32);
    assert!(header_state[1] == ContentType::TransferEncoding as i32);
    assert!(header_state[2] == 0);

    let mut arena = [0u8; 3];
    let mut state = BodyParserState {
        state: BodyParserStatus::ReadChunkSize,
        networking_input: NetworkInput::NeedMoreData,
        current_value: 0,
        arena: &mut arena,
        arena_ptr: 0,
        delimiter_context: DelimiterContext::NoDelimiter,
    };
    parse_http_body(&mut state, chunk, 0);

    assert_eq!(state.state, BodyParserStatus::Success);
    assert_eq!(&state.arena[..state.arena_ptr], b"abc");

    // Два HTTP-чанка в одном входном буфере.
    let chunks = b"3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n";
    let mut arena = [0u8; 5];
    let mut state = BodyParserState {
        state: BodyParserStatus::ReadChunkSize,
        networking_input: NetworkInput::NeedMoreData,
        current_value: 0,
        arena: &mut arena,
        arena_ptr: 0,
        delimiter_context: DelimiterContext::NoDelimiter,
    };
    parse_http_body(&mut state, chunks, 0);
    assert_eq!(state.state, BodyParserStatus::Success);
    assert_eq!(&state.arena[..state.arena_ptr], b"abcde");

    // Тот же вход по сетевым порциям. Каждая получает свежую арену.
    let mut first_arena = [0u8; 3];
    let mut state = BodyParserState {
        state: BodyParserStatus::ReadChunkSize,
        networking_input: NetworkInput::NeedMoreData,
        current_value: 0,
        arena: &mut first_arena,
        arena_ptr: 0,
        delimiter_context: DelimiterContext::NoDelimiter,
    };
    parse_http_body(&mut state, b"3\r", 0);
    assert_eq!(state.state, BodyParserStatus::ExpectChunkLf);
    assert_eq!(state.arena_ptr, 0);

    let mut second_arena = [0u8; 3];
    state.arena = &mut second_arena;
    parse_http_body(&mut state, b"\nab", 0);
    assert_eq!(&state.arena[..state.arena_ptr], b"ab");
    assert_eq!(state.current_value, 1);

    let mut third_arena = [0u8; 3];
    state.arena = &mut third_arena;
    parse_http_body(&mut state, b"c\r\n2\r\nd", 0);
    assert_eq!(&state.arena[..state.arena_ptr], b"cd");
    assert_eq!(state.current_value, 1);

    let mut fourth_arena = [0u8; 3];
    state.arena = &mut fourth_arena;
    parse_http_body(&mut state, b"e\r\n0\r\n\r", 0);
    assert_eq!(&state.arena[..state.arena_ptr], b"e");
    assert_eq!(state.state, BodyParserStatus::ExpectFinalLf);

    let mut fifth_arena = [0u8; 1];
    state.arena = &mut fifth_arena;
    parse_http_body(&mut state, b"\n", 0);
    assert_eq!(state.arena_ptr, 0);
    assert_eq!(state.state, BodyParserStatus::Success);
    let mut bad_header_state = [0i32; 4];
    let bad_request = b"POST /users/123 HTTP/1.1\r\nHost: localhost:9092\r\ntransfer-encoding: chunsked\r\n\r\n3\r\nabc\r\n2\r\nde\r\n0\r\n\r\n";

    let (_, state1) = parse_http_header(bad_request, &mut bad_header_state);
    assert!(state1 == HeaderParserState::Success);
    assert!(bad_header_state[0] == Method::POST as i32);
    assert!(bad_header_state[1] == ContentType::NoContent as i32);
    assert!(bad_header_state[2] == 0);
}
