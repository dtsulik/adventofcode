use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

use crate::custom_error::AocError;

#[derive(PartialEq, PartialOrd)]
struct Region {
    solved: bool,
    chars: Vec<char>,
    len: isize,
}

fn parse_regions(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(
        tag("."),
        map(many1(one_of("#?")), |chars: Vec<char>| Region {
            solved: !chars.contains(&'?'),
            len: chars.len() as isize,
            chars,
        }),
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<Region>, Vec<u16>)> {
    separated_pair(
        parse_regions,
        space1,
        separated_list1(tag(","), complete::u16),
    )
    .parse(input)
}

pub fn factorial(num: isize) -> isize {
    (1..=num).product()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    let rv: usize = input
        .lines()
        .map(|line| {
            let (_, (pipe_layout, journal)) = parse_line(line).expect("Shoud match");

            let number_of_regions = pipe_layout.iter().count();
            let journal_entires = journal.iter().count();
            let sum: u16 = journal.iter().sum();

            pipe_layout.iter().map(|region| if !region.solved {});

            journal
                .iter()
                .map(|group| {
                    println!("Looking for {group}");
                    // calc partitions
                    //

                    // for each partition use combinatorics n.fac()/k.fac() * (n - k).fac()
                    1
                })
                .sum::<usize>()
        })
        .sum();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
