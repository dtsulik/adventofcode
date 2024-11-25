use std::{collections::HashMap, usize};

use crate::custom_error::AocError;

use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let mut has_repeat = false;
            let mut pairs: HashMap<(char, char), (usize, usize)> = HashMap::new();
            let mut pairs_found = false;

            for (current, _next, after_next) in l.chars().tuple_windows() {
                if current == after_next {
                    has_repeat = true
                }
            }
            for (index, (current, next)) in l.chars().tuple_windows().enumerate() {
                if let Some(indexes) = pairs.get(&(current, next)) {
                    if indexes.0 != index
                        && indexes.0 != index + 1
                        && indexes.1 != index
                        && indexes.1 != index + 1
                        && pairs.contains_key(&(current, next))
                    {
                        pairs_found = true;
                    }
                } else {
                    pairs.insert((current, next), (index, index + 1));
                }
            }
            if has_repeat && pairs_found {
                1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", "1")]
    #[case("xxyxx", "1")]
    #[case("uurcxstgmygtbstg", "0")]
    #[case("ieodomkazucvgmuy", "0")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
