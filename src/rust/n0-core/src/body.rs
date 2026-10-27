#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum BodyParserStatus {
    ExpectChunkSize,
    ReadChunkData,
    Success,
    Error,
}
#[derive(Debug)]
pub struct BodyParserState<'a> {
    pub state: BodyParserStatus,
    pub networking_input: NetworkInput,
    pub current_value: usize,
    pub arena: &'a mut [u8],
    pub arena_ptr: usize,
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum NetworkInput {
    ReadingFixedData,
    Timeout,
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
