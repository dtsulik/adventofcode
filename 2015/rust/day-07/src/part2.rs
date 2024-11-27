use crate::custom_error::AocError;
use crate::parser::*;

use std::collections::HashMap;

#[tracing::instrument]
fn build_graph(instructions: Vec<Instruction>) -> HashMap<String, Operation> {
    let mut graph = HashMap::new();
    for instruction in instructions {
        graph.insert(instruction.output, instruction.operation);
    }
    graph
}

#[tracing::instrument]
fn walk(
    wire: &str,
    graph: &HashMap<String, Operation>,
    memo: &mut HashMap<String, u16>,
) -> u16 {
    if let Ok(value) = wire.parse::<u16>() {
        return value;
    }

    if let Some(&value) = memo.get(wire) {
        return value;
    }

    let operation = graph.get(wire).expect("Invalid wire");
    let result = match operation {
        Operation::And(a, b) => walk(a, graph, memo) & walk(b, graph, memo),
        Operation::Or(a, b) => walk(a, graph, memo) | walk(b, graph, memo),
        Operation::LShift(a, shift) => walk(a, graph, memo) << shift,
        Operation::RShift(a, shift) => walk(a, graph, memo) >> shift,
        Operation::Not(a) => !walk(a, graph, memo),
        Operation::Assign(a) => walk(a, graph, memo),
    };

    memo.insert(wire.to_string(), result);
    result
}

// NOTE solving this just meant changing the staic value of xxxx -> b to the part1 output and
// rerunning
#[tracing::instrument]
pub fn process(target: &str, input: &str) -> miette::Result<String, AocError> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let graph = build_graph(instructions);

    let mut memo: HashMap<String, u16> = HashMap::new();

    Ok(walk(target, &graph, &mut memo).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "i",
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "65079"
    )]
    #[case(
        "d",
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "72"
    )]
    #[case(
        "y",
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "456"
    )]
    #[case(
        "x",
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "123"
    )]
    fn test_process(
        #[case] target: &str,
        #[case] input: &str,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(target, input)?);
        Ok(())
    }
}
