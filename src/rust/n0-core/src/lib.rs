pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[derive(PartialEq)]
pub enum HeaderParserState {
    ReqMethod,
    ReqUri,
    ReqVersion,
    EndLF,
    CRLF,
    Error,
    HeaderName,
    HeaderValue,
    Success,
}
#[repr(u8)]
#[derive(PartialEq)]
enum Method {
    PSTAR,
    GET,
    PUT,
    POST,
    PATCH,
} // those will be method codes

//#[repr(u8)]
// enum BodyHeader {
//     FixedContent,
//     ChunkedContent,
// }

#[repr(u8)]
#[derive(PartialEq)]
enum ContentType {
    NoContent,
    ContentLength,
    TransderEncoding,
}

struct MethodData {
    guess: Method,
    matching_count: i8,
}

struct HeaderData {
    content_length_match: usize,
    transfer_encoding_match: usize,
    content_header_failed: bool,
    transfer_encoding_failed: bool,
    content_type: ContentType,
    content_length: i32,
}
struct RecognizingData {
    method: MethodData,
    headers: HeaderData,
}

const CONTENT_LENGTH: &[u8] = b"content-length";
const TRANSFER_ENCODING: &[u8] = b"transfer-encoding";
#[repr(u8)]
pub enum Phase {
    Header = 0,
    Body = 1,
    Finished = 2,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum BodyParserStatus {
    ExpectChunkSize,
    ReadChunkData,
    Success,
    Error,
}

pub struct BodyParserState<'a> {
    pub state: BodyParserStatus,
    pub networking_input: NetworkInput,
    pub current_value: usize,
    pub arena: &'a mut [u8],
    pub arena_ptr: usize,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum NetworkInput {
    ReadingFixedData,
    Timeout,
}

fn wirth_http_header_parser_step(
    b: &u8,
    state: &mut HeaderParserState,
    mut counter: usize,
    rd: &mut RecognizingData,
) -> usize {
    //    print!("byte is {}\n", b);
    match state {
        HeaderParserState::ReqMethod if *b == 32 => {
            counter += 1;
            let m_state = recognize_method(b, rd);
            if m_state == HeaderParserState::Error {
                *state = HeaderParserState::Error;
                return counter;
            }
            *state = HeaderParserState::ReqUri;
            return counter;
        }

        HeaderParserState::ReqMethod if *b != 32 => {
            counter += 1;
            *state = HeaderParserState::ReqMethod;
            let m_state = recognize_method(b, rd);
            if m_state == HeaderParserState::Error {
                *state = HeaderParserState::Error;
                return counter;
            }
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

        HeaderParserState::EndLF if *b == 10 => {
            counter += 1;
            *state = HeaderParserState::Success;
            return counter;
        }
        HeaderParserState::HeaderName if *b == 58 => {
            if !rd.headers.content_header_failed && rd.headers.content_length_match == 14 {
                rd.headers.content_type = ContentType::ContentLength;
            }
            if !rd.headers.transfer_encoding_failed && rd.headers.transfer_encoding_match == 17 {
                rd.headers.content_type = ContentType::TransderEncoding;
            }
            rd.headers.content_length_match = 0;
            rd.headers.transfer_encoding_match = 0;
            rd.headers.content_header_failed = false;
            rd.headers.transfer_encoding_failed = false;
            counter += 1;
            *state = HeaderParserState::HeaderValue;
            return counter;
        }

        HeaderParserState::HeaderName if *b == 13 => {
            counter += 1;
            *state = HeaderParserState::EndLF;
            return counter;
        }
        HeaderParserState::HeaderName => {
            if !rd.headers.content_header_failed
                && rd.headers.content_length_match < CONTENT_LENGTH.len()
                && *b == CONTENT_LENGTH[rd.headers.content_length_match]
            {
                rd.headers.content_length_match += 1;
            } else if rd.headers.transfer_encoding_match < TRANSFER_ENCODING.len()
                && *b == TRANSFER_ENCODING[rd.headers.transfer_encoding_match]
            {
                rd.headers.transfer_encoding_match += 1;
            } else {
                rd.headers.content_length_match = 0;
                rd.headers.transfer_encoding_match = 0;
                rd.headers.content_header_failed = true;
                rd.headers.transfer_encoding_failed = true;
            }
            counter += 1;
            *state = HeaderParserState::HeaderName;

            return counter;
        }

        HeaderParserState::HeaderValue if *b == 13 => {
            counter += 1;
            *state = HeaderParserState::CRLF;
            rd.headers.content_length_match = 0;
            rd.headers.transfer_encoding_match = 0;
            return counter;
        }

        HeaderParserState::HeaderValue => {
            if rd.headers.content_type == ContentType::ContentLength {
                println!("{} vs {}", rd.headers.content_length, *b);
                if *b == 32 {
                    //later we'll deal
                } else if *b > 47 && *b < 58 || *b == 0 {
                    rd.headers.content_length = rd.headers.content_length * 10 + (*b as i32 - 48)
                } else {
                    rd.headers.content_type = ContentType::NoContent;
                    rd.headers.content_header_failed = true;
                }
            }
            counter += 1;
            *state = HeaderParserState::HeaderValue;
            return counter;
        }

        HeaderParserState::Success => {
            *state = HeaderParserState::Success;
            return counter;
        }

        HeaderParserState::Error => {
            *state = HeaderParserState::Error;
            println!("error");
            return counter;
        }

        _other => {
            return counter;
        }
    }
}

fn recognize_method(b: &u8, rd: &mut RecognizingData) -> HeaderParserState {
    //only for get testing
    match rd.method.matching_count {
        0 => match b {
            71 => {
                rd.method.guess = Method::GET;
                rd.method.matching_count += 1
            }
            80 => {
                rd.method.guess = Method::PSTAR;
                rd.method.matching_count += 1
            }
            _ => {}
        },
        1 => match b {
            69 if rd.method.guess == Method::GET => rd.method.matching_count += 1,
            85 if rd.method.guess == Method::PSTAR => {
                rd.method.guess = Method::PUT;
                rd.method.matching_count += 1
            }
            79 if rd.method.guess == Method::PSTAR => {
                rd.method.guess = Method::POST;
                rd.method.matching_count += 1
            }
            65 if rd.method.guess == Method::PSTAR => {
                rd.method.guess = Method::PATCH;
                rd.method.matching_count += 1
            }

            _ if rd.method.guess == Method::PSTAR => return HeaderParserState::Error,

            _ if rd.method.guess == Method::GET => return HeaderParserState::Error,

            _ => {}
        },
        2 => match b {
            84 if rd.method.guess == Method::GET => rd.method.matching_count += 1,
            84 if rd.method.guess == Method::PUT => rd.method.matching_count += 1,
            83 if rd.method.guess == Method::POST => rd.method.matching_count += 1,
            _ if rd.method.guess == Method::GET => return HeaderParserState::Error,
            _ if rd.method.guess == Method::PUT => return HeaderParserState::Error,
            _ => {}
        },
        3 => match b {
            32 if rd.method.guess == Method::GET => {}
            32 if rd.method.guess == Method::PUT => {}
            _ if rd.method.guess == Method::GET => {
                return HeaderParserState::Error;
            }
            _ if rd.method.guess == Method::PUT => {
                return HeaderParserState::Error;
            }
            84 if rd.method.guess == Method::POST => rd.method.matching_count += 1,
            _ if rd.method.guess == Method::POST => {
                return HeaderParserState::Error;
            }
            _ => {}
        },
        4 => match b {
            32 if rd.method.guess == Method::POST => {}
            _ if rd.method.guess == Method::POST => {
                return HeaderParserState::Error;
            }

            _ => {}
        },

        _ => {}
    }
    return HeaderParserState::ReqMethod;
}

fn next_state(state: &mut BodyParserState) {
    match state.state {
        BodyParserStatus::ReadChunkData
            if state.current_value > 0
                && state.networking_input == NetworkInput::ReadingFixedData =>
        {
            state.state = BodyParserStatus::ReadChunkData;
        }
        BodyParserStatus::ReadChunkData
            if state.current_value == 0
                && state.networking_input == NetworkInput::ReadingFixedData =>
        {
            state.state = BodyParserStatus::Success;
        }

        _ => {
            state.state = BodyParserStatus::Error;
        }
    }
}

pub fn parse_http_body_step<'a>(state: &mut BodyParserState<'a>, b: &u8) {
    //let n_state: BodyParserState = next_state(state, network_input, current_value);
    match state.state {
        BodyParserStatus::Success => state.state = BodyParserStatus::Success,
        BodyParserStatus::Error => {
            state.state = BodyParserStatus::Error;
        }
        BodyParserStatus::ReadChunkData => {
            state.current_value -= 1;
            state.state = BodyParserStatus::ReadChunkData;
            state.networking_input = NetworkInput::ReadingFixedData;
            state.arena[state.arena_ptr] = *b;
            state.arena_ptr += 1;
        }
        _ => {}
    }
    next_state(state);
}

pub fn parse_http_header<'a>(buffer: &[u8], i_table: &'a mut [i32]) -> (usize, HeaderParserState) {
    let mut state = HeaderParserState::ReqMethod;
    let mut counter: usize = 0;
    let mut rd = RecognizingData {
        method: MethodData {
            guess: Method::PSTAR,
            matching_count: 0,
        },
        headers: HeaderData {
            content_length_match: 0,
            transfer_encoding_match: 0,
            content_header_failed: false,
            transfer_encoding_failed: false,
            content_type: ContentType::NoContent,
            content_length: 0,
        },
    };

    for b in buffer {
        counter = wirth_http_header_parser_step(b, &mut state, counter, &mut rd);
        if state == HeaderParserState::Success {
            //            println!("counter is {} state is {} \n", counter, counter);
            i_table[0] = rd.method.guess as i32;
            i_table[1] = rd.headers.content_type as i32;
            i_table[2] = rd.headers.content_length;
            break;
        } else if state == HeaderParserState::Error {
            println!("Got error exititng..");
            break;
        }
    }
    return (counter, state);
}

pub fn parse_http_body<'a>(state: &mut BodyParserState<'a>, bytes: &[u8], counter: usize) {
    for b in &bytes[counter..] {
        parse_http_body_step(state, b);
        if state.state == BodyParserStatus::Success {
            println!("finished");
            break;
        } else {
            println!("status is not finished\n");
        }
    }
}

pub fn parse_http_req(
    req_bytes: &[u8],
    arena: &mut [u8],
    i_table: &mut [i32],
    parser_state: &mut [i32],
) {
    let (r, state) = parse_http_header(req_bytes, i_table);
    if state == HeaderParserState::Success {
        //phase here check if content lengt
    } else {
        parser_state[0] = i32::from(Phase::Header as u8);
    }

    let mut state = BodyParserState {
        state: BodyParserStatus::ReadChunkData,
        networking_input: NetworkInput::ReadingFixedData,
        current_value: i_table[2] as usize,
        arena: arena,
        arena_ptr: 0,
    };
    parse_http_body(&mut state, req_bytes, r);
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
