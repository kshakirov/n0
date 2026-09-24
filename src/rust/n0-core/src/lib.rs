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
#[derive(PartialEq)]
enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    PSTAR,
    HEAD,
    PATCH,
} // those will be method codes

#[repr(u8)]
enum BodyHeader {
    FixedContent,
    ChunkedContent,
}

struct MethodData {
    guess: Method,
    matching_count: i8,
}

struct RecognizingData {
    method: MethodData,
}

fn wirth_http_header_parser(
    b: &u8,
    state: &mut HeaderParserState,
    mut counter: i32,
    rd: &mut RecognizingData,
) -> i32 {
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
            recognize_header(b, rd);
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

fn recognize_header(b: &u8, rd: &mut RecognizingData) {
    //only for get testing
    match rd.method.matching_count {
        0 => match b {
            71 => {
                rd.method.guess = Method::GET;
                rd.method.matching_count += 1
            }
            _ => {}
        },
        1 => match b {
            69 => {
                if rd.method.guess == Method::GET {
                    rd.method.matching_count += 1
                }
            }
            _ => {}
        },
        2 => match b {
            69 => {
                if rd.method.guess == Method::GET {
                    rd.method.matching_count += 1
                }
            }
            _ => {}
        },
        3 => match b {
            32 => if rd.method.guess == Method::GET {},
            _ if rd.method.guess == Method::GET => {}
            _ => {}
        },
        _ => {}
    }
}

pub fn parse_http_header<'a>(buffer: &[u8], i_table: &'a mut [i32]) -> &'a mut [i32] {
    for i in &mut *i_table {
        print!("{}", *i);
    }

    let mut state = HeaderParserState::ReqMethod;
    let mut counter = 0;
    let mut rd = RecognizingData {
        method: MethodData {
            guess: Method::PSTAR,
            matching_count: 0,
        },
    };

    for b in buffer {
        counter = wirth_http_header_parser(b, &mut state, counter, &mut rd);
        if state == HeaderParserState::Success {
            println!("counter is {} state is {} \n", counter, counter);
        }
    }

    return i_table;
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
