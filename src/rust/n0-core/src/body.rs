#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum BodyParserStatus {
    ExpectChunkSize,
    ReadChunkData,
    ReadFixedData,
    ReadChunkSize,
    Success,
    Error,
    ExpectChunkCr,
    ExpectChunkLf,
    ExpectFinalCr,
    ExpectFinalLf,
}
#[derive(Debug)]
pub struct BodyParserState<'a> {
    pub state: BodyParserStatus,
    pub networking_input: NetworkInput,
    pub current_value: usize,
    pub arena: &'a mut [u8],
    pub arena_ptr: usize,
    pub delimiter_context: DelimiterContext,
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum NetworkInput {
    ReadingFixedData,
    Timeout,
    NeedMoreData,
    ChunkSizeDigitAccepted,
    ZeroChunkSizeTerminated,
    ChunkSizeTerminated,
    CrAccepted,
    LfAccepted,
    FixedBodyByteAccepted,
    ChunkDataByteAccepted,
    InvalidInput,
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum DelimiterContext {
    NoDelimiter,
    AfterSize,
    AfterData,
    AfterZero,
}

fn next_state(state: &mut BodyParserState) {
    match state.state {
        BodyParserStatus::ReadFixedData
            if state.current_value > 0
                && state.networking_input == NetworkInput::ReadingFixedData =>
        {
            state.state = BodyParserStatus::ReadFixedData;
        }
        BodyParserStatus::ReadFixedData
            if state.current_value == 0
                && state.networking_input == NetworkInput::ReadingFixedData =>
        {
            state.state = BodyParserStatus::Success;
        }
        BodyParserStatus::ReadChunkSize
            if state.networking_input == NetworkInput::ChunkSizeDigitAccepted =>
        {
            state.state = BodyParserStatus::ReadChunkSize
        }

        BodyParserStatus::ReadChunkSize
            if state.networking_input == NetworkInput::ChunkSizeTerminated =>
        {
            state.state = BodyParserStatus::ExpectChunkLf
        }
        BodyParserStatus::ReadChunkSize
            if state.networking_input == NetworkInput::ZeroChunkSizeTerminated =>
        {
            state.state = BodyParserStatus::ExpectChunkLf
        }

        BodyParserStatus::ExpectChunkLf
            if state.networking_input == NetworkInput::LfAccepted
                && state.delimiter_context == DelimiterContext::AfterSize =>
        {
            state.state = BodyParserStatus::ReadChunkData;
        }

        BodyParserStatus::ExpectChunkLf
            if state.networking_input == NetworkInput::LfAccepted
                && state.delimiter_context == DelimiterContext::AfterData =>
        {
            state.state = BodyParserStatus::ReadChunkSize;
        }

        BodyParserStatus::ExpectChunkLf
            if state.networking_input == NetworkInput::LfAccepted
                && state.delimiter_context == DelimiterContext::AfterZero =>
        {
            state.state = BodyParserStatus::ExpectFinalCr;
        }

        BodyParserStatus::ReadChunkData
            if state.networking_input == NetworkInput::ChunkDataByteAccepted
                && state.current_value > 1 =>
        {
            state.state = BodyParserStatus::ReadChunkData;
        }
        BodyParserStatus::ReadChunkData
            if state.networking_input == NetworkInput::ChunkDataByteAccepted
                && state.current_value == 1 =>
        {
            state.state = BodyParserStatus::ExpectChunkCr;
        }

        BodyParserStatus::ExpectChunkCr if state.networking_input == NetworkInput::CrAccepted => {
            state.state = BodyParserStatus::ExpectChunkLf;
        }
        BodyParserStatus::ExpectFinalCr if state.networking_input == NetworkInput::CrAccepted => {
            state.state = BodyParserStatus::ExpectFinalLf;
        }
        BodyParserStatus::ExpectFinalLf if state.networking_input == NetworkInput::LfAccepted => {
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
        BodyParserStatus::ReadFixedData => {
            state.current_value -= 1;
            state.state = BodyParserStatus::ReadFixedData;
            state.networking_input = NetworkInput::ReadingFixedData;
            state.arena[state.arena_ptr] = *b;
            state.arena_ptr += 1;
        }
        BodyParserStatus::ReadChunkSize => {
            if *b == b'\r' {
                state.networking_input = if state.current_value == 0 {
                    NetworkInput::ZeroChunkSizeTerminated
                } else {
                    NetworkInput::ChunkSizeTerminated
                };
            } else if *b > b'0' && *b < b'd' {
                let v = from_ascii_to_hex(*b);
                if v > -1 {
                    state.current_value = 16 * state.current_value + (v as usize);
                    state.networking_input = NetworkInput::ChunkSizeDigitAccepted;
                } else {
                    state.state = BodyParserStatus::Error;
                }
            } else {
                state.state = BodyParserStatus::Error;
            }
        }
        BodyParserStatus::ReadChunkData => {
            state.current_value -= 1;
            state.state = BodyParserStatus::ReadChunkData;
            state.networking_input = NetworkInput::ChunkDataByteAccepted;
            state.arena[state.arena_ptr] = *b;
            state.arena_ptr += 1;
        }

        BodyParserStatus::ExpectChunkCr if *b == 13 => {
            state.networking_input = NetworkInput::CrAccepted;
        }
        BodyParserStatus::ExpectChunkCr => {
            state.networking_input = NetworkInput::InvalidInput;
        }

        BodyParserStatus::ExpectChunkLf if *b == 10 => {
            state.networking_input = NetworkInput::LfAccepted;
        }
        BodyParserStatus::ExpectChunkLf => {
            state.networking_input = NetworkInput::InvalidInput;
        }
        BodyParserStatus::ExpectFinalCr if *b == 13 => {
            state.networking_input = NetworkInput::CrAccepted;
        }
        BodyParserStatus::ExpectFinalCr => {
            state.networking_input = NetworkInput::InvalidInput;
        }
        BodyParserStatus::ExpectFinalLf if *b == 10 => {
            state.networking_input = NetworkInput::LfAccepted;
        }
        BodyParserStatus::ExpectFinalLf => {
            state.networking_input = NetworkInput::InvalidInput;
        }

        _ => {}
    }
    next_state(state);
}

fn from_ascii_to_hex(c: u8) -> i32 {
    let v: i32;
    if c >= b'0' && c <= b'9' {
        v = i32::from(c as u8) - 48
    } else if c >= b'A' && c <= b'F' {
        v = i32::from(c as u8) - 65 + 10
    } else if c >= b'a' && c <= b'f' {
        v = i32::from(c as u8) - 97 + 10
    } else {
        v = -1
    };
    return v;
}

pub fn parse_http_body<'a>(state: &mut BodyParserState<'a>, bytes: &[u8], counter: usize) {
    state.arena_ptr = 0;
    for b in &bytes[counter..] {
        parse_http_body_step(state, b);
        if state.state == BodyParserStatus::Success {
            //println!("finished");
            break;
        } else {
            //println!("status is not finished\n");
        }
    }
}
