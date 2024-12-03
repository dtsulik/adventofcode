use crate::custom_error::AocError;

use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut dont = false;
    Ok(input
        .lines()
        .map(|l| {
            let re = Regex::new(r"mul\((\d+),(\d+)\)|(don\'t\(\))|(do\(\))").unwrap();

            re.captures_iter(l)
                .filter_map(|cap| {
                    if cap.get(1).is_some() {
                        if dont {
                            None
                        } else {
                            let lhs = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                            let rhs = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                            Some((lhs, rhs))
                        }
                    } else if cap.get(3).is_some() {
                        dont = true;
                        None
                    } else if cap.get(4).is_some() {
                        dont = false;
                        None
                    } else {
                        println!("if this happen -<>>>>>>>>>>>>>>>");
                        None
                    }
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
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        "48"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
