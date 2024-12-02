use crate::custom_error::AocError;

#[derive(PartialEq, Eq)]
pub enum Direction {
    INC,
    DEC,
    INIT1,
    INIT2,
}

use Direction::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let res = l.split(' ').try_fold((INIT1, 0), |(state, prev), num_str| {
                let num: i32 = num_str.parse().unwrap();
                match state {
                    INIT1 => Some((INIT2, num)),
                    INIT2 => {
                        if prev > num && prev - num <= 3 {
                            Some((DEC, num))
                        } else if num > prev && num - prev <= 3 {
                            Some((INC, num))
                        } else {
                            None
                        }
                    }
                    INC => {
                        if num > prev && num - prev <= 3 {
                            Some((INC, num))
                        } else {
                            None
                        }
                    }
                    DEC => {
                        if prev > num && prev - num <= 3 {
                            Some((DEC, num))
                        } else {
                            None
                        }
                    }
                }
            });
            if res.is_none() {
                0
            } else {
                1
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
    #[case("7 6 4 2 1", "1")]
    #[case("1 2 7 8 9", "0")]
    #[case(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        "2"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
