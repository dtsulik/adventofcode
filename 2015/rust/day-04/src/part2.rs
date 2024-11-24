use md5::{Digest, Md5};

use crate::custom_error::AocError;

#[tracing::instrument]
fn check_num(input: &str) -> bool {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    result[0] == 0 && result[1] == 0 && result[2]  == 0
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
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    fn test_process(#[case] _input: &str, #[case] _expected: &str) -> miette::Result<()> {
        Ok(())
    }
}
