use crate::custom_error::AocError;

use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use nom_supreme::ParserExt;
fn numbers(input: &str) -> IResult<&str, usize> {
    is_not("0123456789")
        .precedes(
            separated_list1(space1, digit1).map(|digits| digits.join("").parse::<usize>().unwrap()),
        )
        .parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (time, distance)) = separated_pair(numbers, line_ending, numbers)
        .parse(input)
        .expect("failed parsing input");

    let rv = (0..time)
        .filter_map(|s| {
            let d = (time - s) * s;
            if d > distance {
                Some(d)
            } else {
                None
            }
        })
        .count();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}

