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

        let mut front_char = 'z';
        let mut back_char = 'z';

        let mut foundfront = false;
        let mut foundback = false;

        while !foundfront || !foundback {
            if !foundfront {
                front_char = bytes[front] as char;
                if front_char.is_ascii_digit() {
                    foundfront = true;
                    continue;
                }
                if bytes[front] == b'o' {
                    if bytes.len() - front >= 3 {
                        if bytes[front + 1] == b'n' && bytes[front + 2] == b'e' {
                            front_char = '1';
                            foundfront = true;
                        }
                    }
                }
                if bytes[front] == b't' {
                    if bytes.len() - front >= 3 {
                        if bytes[front + 1] == b'w' && bytes[front + 2] == b'o' {
                            front_char = '2';
                            foundfront = true;
                        }
                    }
                    if bytes.len() - front >= 5 {
                        if bytes[front + 1] == b'h'
                            && bytes[front + 2] == b'r'
                            && bytes[front + 3] == b'e'
                            && bytes[front + 4] == b'e'
                        {
                            front_char = '3';
                            foundfront = true;
                        }
                    }
                }
                if bytes[front] == b'f' {
                    if bytes.len() - front >= 4 {
                        if bytes[front + 1] == b'o'
                            && bytes[front + 2] == b'u'
                            && bytes[front + 3] == b'r'
                        {
                            front_char = '4';
                            foundfront = true;
                        }
                    }
                    if bytes.len() - front >= 4 {
                        if bytes[front + 1] == b'i'
                            && bytes[front + 2] == b'v'
                            && bytes[front + 3] == b'e'
                        {
                            front_char = '5';
                            foundfront = true;
                        }
                    }
                }
                if bytes[front] == b's' {
                    if bytes.len() - front >= 3 {
                        if bytes[front + 1] == b'i' && bytes[front + 2] == b'x' {
                            front_char = '6';
                            foundfront = true;
                        }
                    }
                    if bytes.len() - front >= 5 {
                        if bytes[front + 1] == b'e'
                            && bytes[front + 2] == b'v'
                            && bytes[front + 3] == b'e'
                            && bytes[front + 4] == b'n'
                        {
                            front_char = '7';
                            foundfront = true;
                        }
                    }
                }
                if bytes[front] == b'e' {
                    if bytes.len() - front >= 5 {
                        if bytes[front + 1] == b'i'
                            && bytes[front + 2] == b'g'
                            && bytes[front + 3] == b'h'
                            && bytes[front + 4] == b't'
                        {
                            front_char = '8';
                            foundfront = true;
                        }
                    }
                }
                if bytes[front] == b'n' {
                    if bytes.len() - front >= 4 {
                        if bytes[front + 1] == b'i'
                            && bytes[front + 2] == b'n'
                            && bytes[front + 3] == b'e'
                        {
                            front_char = '9';
                            foundfront = true;
                        }
                    }
                }
                if !foundfront {
                    front += 1;
                }
            }

            if !foundback {
                back_char = bytes[back] as char;
                if back_char.is_ascii_digit() {
                    foundback = true;
                    continue;
                }
                if bytes[back] == b'e' {
                    if back >= 3 {
                        if bytes[back - 1] == b'n' && bytes[back - 2] == b'o' {
                            back_char = '1';
                            foundback = true;
                            continue;
                        }
                    }
                    if back >= 5 {
                        if bytes[back - 1] == b'e'
                            && bytes[back - 2] == b'r'
                            && bytes[back - 3] == b'h'
                            && bytes[back - 4] == b't'
                        {
                            back_char = '3';
                            foundback = true;
                            continue;
                        }
                    }
                    if back >= 4 {
                        if bytes[back - 1] == b'v'
                            && bytes[back - 2] == b'i'
                            && bytes[back - 3] == b'f'
                        {
                            back_char = '5';
                            foundback = true;
                            continue;
                        }
                    }
                    if back >= 4 {
                        if bytes[back - 1] == b'n'
                            && bytes[back - 2] == b'i'
                            && bytes[back - 3] == b'n'
                        {
                            back_char = '9';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if bytes[back] == b'o' {
                    if back >= 3 {
                        if bytes[back - 1] == b'w' && bytes[back - 2] == b't' {
                            back_char = '2';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if bytes[back] == b'r' {
                    if back >= 4 {
                        if bytes[back - 1] == b'u'
                            && bytes[back - 2] == b'o'
                            && bytes[back - 3] == b'f'
                        {
                            back_char = '4';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if bytes[back] == b'x' {
                    if back >= 3 {
                        if bytes[back - 1] == b'i' && bytes[back - 2] == b's' {
                            back_char = '6';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if bytes[back] == b'n' {
                    if back >= 5 {
                        if bytes[back - 1] == b'e'
                            && bytes[back - 2] == b'v'
                            && bytes[back - 3] == b'e'
                            && bytes[back - 4] == b's'
                        {
                            back_char = '7';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if bytes[back] == b't' {
                    if back >= 5 {
                        if bytes[back - 1] == b'h'
                            && bytes[back - 2] == b'g'
                            && bytes[back - 3] == b'i'
                            && bytes[back - 4] == b'e'
                        {
                            back_char = '8';
                            foundback = true;
                            continue;
                        }
                    }
                }
                if !foundback {
                    back -= 1;
                }
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
