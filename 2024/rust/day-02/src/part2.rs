use crate::custom_error::AocError;

use crate::part1::Direction::*;

fn is_safe(nums: &Vec<i32>) -> i32 {
    let res = nums
        .iter()
        .try_fold((INIT1, 0), |(state, prev), &num| match state {
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
        });
    if res.is_none() {
        0
    } else {
        1
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let nums = l.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
            let res = is_safe(&nums);
            if res == 1 {
                res
            } else {
                let combinations: Vec<Vec<i32>> = (0..nums.len())
                    .map(|i| {
                        nums.iter()
                            .enumerate()
                            .filter(|&(j, _)| j != i)
                            .map(|(_, &val)| val)
                            .collect()
                    })
                    .collect();
                let mut res = 0;
                for combination in combinations {
                    res = is_safe(&combination);
                    if res == 1 {break;}
                }
                res
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
        "4"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
