use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use nom_supreme::ParserExt;

use crate::custom_error::AocError;

const START: &str = "AAA";
const END: &str = "ZZZ";

fn parse_desert_map(input: &str) -> IResult<&str, (&str, &str)> {
    tag("(")
        .precedes(terminated(
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ))
        .parse(input)
}

fn parse_directions(input: &str) -> IResult<&str, &str> {
    terminated(alpha1, multispace1).parse(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {

    let (input, directions) = parse_directions(input).expect("Failed to get directions");
    println!("Using directions: {directions}");

    let desert_map: HashMap<&str, (&str, &str)> = input
        .lines()
        .map(|line| {
            println!("working on line {line}");
            let (_, rv) = separated_pair(alpha1, tag(" = "), parse_desert_map)
                .parse(line)
                .unwrap();
            println!("nom parsing result {:?}", rv);
            rv
        })
        .collect();

    let mut rv = 0;
    let mut current_node = START;
    for d in directions.chars().cycle() {
        println!("starting on node {current_node}");
        let node = match d {
            'R' => desert_map.get(current_node).unwrap().1,
            'L' => desert_map.get(current_node).unwrap().0,
            _ => panic!("Should never happen"),
        };
        println!("Based on direction {d}, moving to node {node}");
        rv += 1;
        if node.starts_with(END) {
            break;
        } else {
            current_node = node;
        }
    }

    println!("Desert map: {:?}", desert_map);

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_llr() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_rl() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
