use nom::{
    Parser,
    character::complete::one_of,
    combinator::{all_consuming, map},
    multi::many1,
};

#[macro_export]
macro_rules! test_len_case {
    ($name:ident: $enc:expr => [
        $(
            ($in:expr, $out:expr $(,)?)
        ),+ $(,)?
    ]) => {
        #[test]
        fn $name() {
            for (seq, exp) in [$(($in, $out)),*] {
                assert_eq!($enc.encode(&seq).as_ref(), &exp);
            }
        }
    };
}

#[inline]
pub fn parse_bits(input: &str) -> Option<Box<[u8]>> {
    all_consuming(many1(map(
        one_of::<_, _, nom::error::Error<_>>("01"),
        |src| if src == '1' { 1 } else { 0 },
    )))
    .parse(input)
    .map(|(_, bytes)| bytes.into_boxed_slice())
    .ok()
}

#[cfg(test)]
mod tests {
    use super::parse_bits;

    #[test]
    fn test_bits_parser_with_only_bits() {
        let input = "111001010011010111010111010101010110";
        let result = parse_bits(input);

        assert!(result.is_some());
        assert_eq!(
            Box::from([
                1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1,
                0, 1, 0, 1, 0, 1, 1, 0
            ]),
            result.unwrap()
        );
    }

    #[test]
    fn test_bits_parser_falis_with_no_bits_data() {
        for input in ["", "abc135", "09"] {
            assert!(parse_bits(input).is_none());
        }
    }
}
