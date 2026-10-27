use crate::body::{parse_http_body, BodyParserState, BodyParserStatus, NetworkInput};
use crate::header::{parse_http_header, HeaderParserState};

#[repr(u8)]
pub enum Phase {
    Header = 0,
    Body = 1,
    Finished = 2,
}

const CONTENT_LENGTH: usize = 1;
const PHASE: usize = 0;
const CURRENT_VALUE: usize = 2;

pub fn parse_http_req(
    req_bytes: &[u8],
    i_table: &mut [i32],
    parser_state: &mut [i32],
    body_parser_state: &mut BodyParserState,
) {
    let mut r: usize = 0;

    if parser_state[PHASE] == i32::from(Phase::Header as u8) {
        let state;
        (r, state) = parse_http_header(req_bytes, i_table);
        if state == HeaderParserState::Success {
            //phase here check if content lengti
            if i_table[CONTENT_LENGTH] > 0 {
                parser_state[PHASE] = i32::from(Phase::Body as u8);
                body_parser_state.state = BodyParserStatus::ReadChunkData;
                body_parser_state.networking_input = NetworkInput::ReadingFixedData;
                body_parser_state.current_value = i_table[CURRENT_VALUE] as usize;
                //                body_parser_state.arena = [0u8; 10].as_mut_ptr();
                body_parser_state.arena_ptr = 0;
                parse_http_body(body_parser_state, req_bytes, r);
                if body_parser_state.state == BodyParserStatus::Success {
                    parser_state[PHASE] = i32::from(Phase::Finished as u8);
                }
                return;
            } else {
                parser_state[PHASE] = i32::from(Phase::Finished as u8);
                return;
            }
        } else {
            parser_state[0] = i32::from(Phase::Header as u8);
            return;
        }
    }

    if parser_state[0] == i32::from(Phase::Body as u8) {
        parse_http_body(body_parser_state, req_bytes, r);
        if body_parser_state.state == BodyParserStatus::Success {
            parser_state[PHASE] = i32::from(Phase::Finished as u8);
        }

        return;
    }

    //hrere will be body state analysys
}
