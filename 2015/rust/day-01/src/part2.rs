use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .chars()
        .enumerate()
        .scan(0, |floor, (index, c)| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some((*floor, index + 1))
        })
        .find_map(|(floor, index)| {
            if floor == -1 {
                Some(index.to_string())
            } else {
                None
            }
        })
        .unwrap_or(0.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(")", "1")]
    #[case("()())", "5")]
    #[case(")()()()", "1")]
    #[case("()())()()(())()))()", "5")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}

