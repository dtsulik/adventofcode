use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut rv: i64 = 0;

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let mut ns = String::new();
        let bytes = line.as_bytes();

        let mut front = 0;
        let mut back = line.len() - 1;

        let mut front_char = 'f';
        let mut back_char = 'f';

        while !front_char.is_ascii_digit() || !back_char.is_ascii_digit() {
            front_char = bytes[front] as char;
            back_char = bytes[back] as char;

            if !front_char.is_ascii_digit() {
                front += 1;
            }
            if !back_char.is_ascii_digit() {
                back -= 1;
            }
        }
        ns.push(front_char);
        ns.push(back_char);

        let n = ns.parse::<i64>()?;
        rv += n;
    }
    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
asdfasf4fd2
a1b2c3d4e5f
treb7uchet
5nnine";
        assert_eq!("239", process(input)?);
        Ok(())
    }
}
