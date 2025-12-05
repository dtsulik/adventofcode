use crate::custom_error::AocError;

fn parse_line(input: &str) -> Result<i32, AocError> {
    let mut chars = input.chars();
    let letter = chars.next().ok_or(AocError::ParseError("Empty line"))?;
    let number: i32 = chars
        .as_str()
        .parse()
        .map_err(|_| AocError::ParseError("Invalid number"))?;

    match letter {
        'L' => Ok(-number),
        'R' => Ok(number),
        _ => Err(AocError::ParseError("Invalid direction")),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut acc = 50;
    let mut code = 0;

    for line in input.lines() {
        let amount = parse_line(line).unwrap();
        let rem = amount % 100;
        acc += rem;
        acc %= 100;
        if acc == 0 {
            code += 1;
        }
    }

    Ok(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        "3"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
