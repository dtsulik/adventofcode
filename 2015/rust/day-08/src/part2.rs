use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (sum_chars, sum_mem_chars) = input
        .lines()
        .map(|l| {
            let num_chars: i32 = l.chars().count() as i32;
            let mut num_mem_chars = 6;
            let mut i = 1;

            let charv: Vec<char> = l.chars().collect();

            while i < charv.len() - 1 {
                if charv[i] == '\\' && i + 1 < charv.len() - 1 {
                    match charv[i + 1] {
                        '"' | '\\' => {
                            num_mem_chars += 4;
                            i += 2;
                            continue;
                        }
                        'x' if i + 3 < charv.len() => {
                            num_mem_chars += 5;
                            i += 4;
                            continue;
                        }
                        _ => {}
                    }
                }
                num_mem_chars += 1;
                i += 1;
            }

            (num_chars, num_mem_chars)
        })
        .fold((0, 0), |(sum_chars, sum_mem_chars), (chars, mem_chars)| {
            (sum_chars + chars, sum_mem_chars + mem_chars)
        });

    println!("sum_chars {sum_chars} mem_chars {sum_mem_chars}");

    Ok((sum_mem_chars - sum_chars).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"",
        "19"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
