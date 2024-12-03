use crate::custom_error::AocError;

use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            re.captures_iter(l)
                .filter_map(|cap| {
                    let lhs = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let rhs = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    Some((lhs, rhs))
                })
                .map(|(lhs, rhs)| lhs * rhs)
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "161"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
