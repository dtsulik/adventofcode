use crate::custom_error::AocError;

fn is_valid(input: &str) -> bool {
    false
}

fn next_pw(input: &str) -> &str {

    ""
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    



    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    fn test_process(#[case] input: &str, #[case] expected: bool) -> miette::Result<()> {
        assert_eq!(expected, is_valid(input));
        Ok(())
    }
    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_process_next(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, next_pw(input));
        Ok(())
    }
}
