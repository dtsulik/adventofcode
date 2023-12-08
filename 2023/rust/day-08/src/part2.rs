use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use nom_supreme::ParserExt;

use rayon::prelude::*;

use crate::custom_error::AocError;

fn parse_desert_map(input: &str) -> IResult<&str, (&str, &str)> {
    tag("(")
        .precedes(terminated(
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ))
        .parse(input)
}

fn parse_directions(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1).parse(input)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (input, directions) = parse_directions(input).expect("Failed to get directions");

    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let desert_map: HashMap<&str, (&str, &str)> = input
        .lines()
        .map(|line| {
            let (_, rv) = separated_pair(alphanumeric1, tag(" = "), parse_desert_map)
                .parse(line)
                .unwrap();
            if rv.0.ends_with("Z") {
                ends.push(rv.0);
            }
            if rv.0.ends_with("A") {
                starts.push(rv.0)
            }

            rv
        })
        .collect();

    let moves = starts
        .par_iter()
        .map(|s| {
            let mut current_node = *s;
            let mut counter = 0;
            for d in directions.chars().cycle() {
                let node = match d {
                    'R' => desert_map.get(current_node).unwrap().1,
                    'L' => desert_map.get(current_node).unwrap().0,
                    _ => panic!("Should never happen"),
                };
                counter += 1;
                if node.ends_with("Z") {
                    break;
                } else {
                    current_node = node;
                }
            }
            counter
        })
        .collect::<HashSet<u64>>();

    let rv = moves.iter().fold(1, |acc, &num| lcm(acc, num));

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_rl() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}





