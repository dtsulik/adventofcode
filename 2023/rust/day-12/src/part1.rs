// use indicatif::ParallelProgressIterator;
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::is_a,
    character::complete::{self, char as tag_char, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use rayon::prelude::*;

use crate::custom_error::AocError;

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<u64>)> {
    separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag_char(','), complete::u64),
    )
    .parse(input)
}

pub fn factorial(num: isize) -> isize {
    (1..=num).product()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    //     let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1";

    let rv: usize = input
        // .par_lines()
        .lines()
        .map(|line| {
            let (_, (row, journal)) =
                parse_line(line).expect("Should get a row with its journal entries");

            let unknown_count = row.chars().filter(|&c| c == '?').count();

            let all_possible_combinations: Vec<String> =
                repeat_n([".", "#"].into_iter(), unknown_count)
                    .multi_cartesian_product()
                    .map(|s| s.join(""))
                    .collect();

            all_possible_combinations
                // .iter()
                .par_iter()
                // .progress_count(all_possible_combinations.len() as u64)
                .map(|layout| {
                    let mut it = layout.chars();
                    let possible_solution: String = line
                        .chars()
                        .map(|c| {
                            if c == '?' {
                                it.next().expect("This should never happen")
                            } else {
                                c
                            }
                        })
                        .collect();
                    let possible_journal: Vec<u64> = possible_solution
                        .chars()
                        .group_by(|&c| c == '#')
                        .into_iter()
                        .filter_map(|(check, group)| {
                            if check {
                                Some(group.into_iter().count() as u64)
                            } else {
                                None
                            }
                        })
                        .collect();

                    if possible_journal == journal {
                        1
                    } else {
                        0
                    }
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
