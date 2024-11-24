use md5::{Digest, Md5};

use crate::custom_error::AocError;

#[tracing::instrument]
fn check_num(input: &str) -> bool {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    result[0] == 0 && result[1] == 0 && result[2] & 0xF0 == 0
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut num: u64 = 0;

    let input = input.trim();

    loop {
        let input_str = format!("{}{}", input, num);
        if check_num(&input_str) {
            break Ok(num.to_string());
        }
        num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", "609043")]
    #[case("pqrstuv", "1048970")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        let inputs = format!("{}{}", input, expected);
        if check_num(&inputs) {
            Ok(())
        } else {
            assert_eq!(1, 2);
            Ok(())
        }
    }
}
