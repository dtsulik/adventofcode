use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub enum Operation {
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u32),
    RShift(String, u32),
    Assign(String),
}

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub output: String,
}

fn parse_not(input: &str) -> IResult<&str, Operation> {
    let (input, operand) = preceded(tag("NOT "), alphanumeric1)(input)?;
    Ok((input, Operation::Not(operand.to_string())))
}

fn parse_and(input: &str) -> IResult<&str, Operation> {
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(" AND "), alphanumeric1)(input)?;
    Ok((input, Operation::And(left.to_string(), right.to_string())))
}

fn parse_or(input: &str) -> IResult<&str, Operation> {
    let (input, (left, right)) = separated_pair(alphanumeric1, tag(" OR "), alphanumeric1)(input)?;
    Ok((input, Operation::Or(left.to_string(), right.to_string())))
}

fn parse_lshift(input: &str) -> IResult<&str, Operation> {
    let (input, (left, shift)) =
        separated_pair(alphanumeric1, tag(" LSHIFT "), alphanumeric1)(input)?;
    Ok((
        input,
        Operation::LShift(left.to_string(), shift.parse::<u32>().unwrap()),
    ))
}

fn parse_rshift(input: &str) -> IResult<&str, Operation> {
    let (input, (left, shift)) =
        separated_pair(alphanumeric1, tag(" RSHIFT "), alphanumeric1)(input)?;
    Ok((
        input,
        Operation::RShift(left.to_string(), shift.parse::<u32>().unwrap()),
    ))
}

fn parse_assign(input: &str) -> IResult<&str, Operation> {
    let (input, value) = alphanumeric1(input)?;
    Ok((input, Operation::Assign(value.to_string())))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        parse_not,
        parse_and,
        parse_or,
        parse_lshift,
        parse_rshift,
        parse_assign,
    ))(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, (operation, _, output)) =
        tuple((parse_operation, tag(" -> "), alphanumeric1))(input)?;
    Ok((
        input,
        Instruction {
            operation,
            output: output.to_string(),
        },
    ))
}
