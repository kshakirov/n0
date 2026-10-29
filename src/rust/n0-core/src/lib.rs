mod body;
mod header;
mod request;

pub use body::{
    parse_http_body, parse_http_body_step, BodyParserState, BodyParserStatus, DelimiterContext,
    NetworkInput,
};
pub use header::{parse_http_header, HeaderParserState};
pub use request::{parse_http_req, Phase};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
