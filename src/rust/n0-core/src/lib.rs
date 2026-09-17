pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse_http_header(buffer: &mut [u8]) -> &mut [u8] {
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
