pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[derive(PartialEq)]
enum HeaderParserState {
    ReqMethod,
    ReqUri,
    ReqVersion,
    EndLF,
    CRLF,
    Error,
    HeaderName,
    Success,
}
#[repr(u8)]
enum Method {
    GET,
    PUT,
    POST,
    DELETE,
} // those will be method codes

#[repr(u8)]
enum BodyHeader {
    FixedContent,
    ChunkedContent,
}

fn wirth_http_header_parser(b: &mut u8, state: &mut HeaderParserState, mut counter: i32) -> i32 {
    //    print!("byte is {}\n", b);
    match state {
        HeaderParserState::ReqMethod if *b == 32 => {
            counter += 1;
            *state = HeaderParserState::ReqUri;
            return counter;
        }

        HeaderParserState::ReqMethod if *b != 32 => {
            counter += 1;
            *state = HeaderParserState::ReqMethod;
            return counter;
        }

        HeaderParserState::ReqUri if *b == 32 => {
            counter += 1;
            *state = HeaderParserState::ReqVersion;
            return counter;
        }

        HeaderParserState::ReqUri => {
            counter += 1;
            *state = HeaderParserState::ReqUri;
            return counter;
        }

        HeaderParserState::ReqVersion if *b == 13 => {
            counter += 1;
            *state = HeaderParserState::CRLF;
            return counter;
        }

        HeaderParserState::ReqVersion => {
            counter += 1;
            *state = HeaderParserState::ReqVersion;
            return counter;
        }

        HeaderParserState::CRLF if *b == 10 => {
            counter += 1;
            *state = HeaderParserState::HeaderName;
            return counter;
        }

        HeaderParserState::HeaderName if *b == 13 => {
            counter += 1;
            *state = HeaderParserState::EndLF;
            return counter;
        }
        HeaderParserState::EndLF if *b == 10 => {
            counter += 1;
            *state = HeaderParserState::Success;
            return counter;
        }

        HeaderParserState::Success => {
            *state = HeaderParserState::Success;
            return counter;
        }

        _other => {
            return counter;
        }
    }
}

pub fn parse_http_header(buffer: &mut [u8]) -> &mut [u8] {
    let bytes: &mut [u8] = buffer.as_mut();
    let mut state = HeaderParserState::ReqMethod;
    let mut counter = 0;
    for b in bytes {
        counter = wirth_http_header_parser(b, &mut state, counter);
        if state == HeaderParserState::Success {
            println!("counter is {} state is {} \n", counter, counter);
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
