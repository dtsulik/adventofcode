use nom::{
    IResult, Parser,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

use crate::custom_error::AocError;

fn range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u64>()),
        char('-'),
        map_res(digit1, |s: &str| s.parse::<u64>()),
    )
    .parse(input)
}

fn ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(char(','), range).parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = input.trim();
    // println!("inputs: {:?}", input);

    let res = match ranges(input) {
        Ok((_, product_id_ranges)) => {
            product_id_ranges
                .iter()
                .map(|r| {
                    let nums = r.0..=r.1;
                    nums.map(|n| {
                        let s = n.to_string();
                        let s_len = s.len();
                        if s_len % 2 == 0 {
                            let mid = s_len / 2;
                            let (left, right) = s.split_at(mid);
                            if left == right {
                                println!("Num is invalid: {}", s);
                                n
                            } else {
                                0
                            }
                        } else {
                            // let all_same = s.chars().all(|c| c == s.chars().next().unwrap());
                            // if all_same {
                            //     println!("Num is invalid: {}", s);
                            //     n
                            // } else {
                            //     0
                            // }
                            0
                        }
                    })
                    .sum::<u64>()
                })
                .sum()
        }
        Err(e) => {
            eprintln!("Error parsing input {:?}", e);
            0
        }
    };

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        "1227775554"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
