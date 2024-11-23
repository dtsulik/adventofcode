use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    Ok(input.chars().map(|x| match x {
        '(' => 1,
        ')' => -1,
        _ => 0
    }).sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", "0")]
    #[case("()()", "0")]
    #[case("(((", "3")]
    #[case("(()(()(", "3")]
    #[case("))(((((", "3")]
    #[case("())", "-1")]
    #[case("))(", "-1")]
    #[case(")))", "-3")]
    #[case(")())())", "-3")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
