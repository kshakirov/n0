pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

enum HeaderParserState {
    ReqMethod,
    ReqUri,
    ReqVersion,
    CR,
    CRLF,
}

pub fn parse_http_header(buffer: &mut [u8]) -> &mut [u8] {
    let bytes: &mut [u8] = buffer.as_mut();
    let mut state = &HeaderParserState::ReqMethod;
    let mut counter = 0;
    for b in bytes {
        match state {
            HeaderParserState::ReqMethod if *b == 13 => {
                counter += 1;
                state = &HeaderParserState::ReqUri
            }
            _other => (),
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
