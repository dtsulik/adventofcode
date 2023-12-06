use crate::custom_error::AocError;

use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use nom_supreme::ParserExt;

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, (times, distances)) = separated_pair(numbers, line_ending, numbers)
        .parse(input)
        .expect("failed parsing input");

    let rv = times
        .into_iter()
        .zip(distances)
        .map(|(time, top_distance)| {
            (0..time)
                .filter_map(|s| {
                    let d = (time - s) * s;
                    if d > top_distance {
                        Some(d)
                    } else {
                        None
                    }
                })
                .count()
        })
        .product::<usize>();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
