pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

enum HeaderParserState {
    ReqMethod,
    ReqUri,
    ReqVersion,
    CR,
    CRLF,
    Error,
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
        println!("{}\n", counter);
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
