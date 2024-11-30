use crate::custom_error::AocError;

#[tracing::instrument]
fn look_and_say(input: &str) -> String {
    let mut rv = String::new();

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        let mut group = String::from(c);

        while let Some(&next) = chars.peek() {
            if next == c {
                group.push(chars.next().unwrap());
            } else {
                break;
            }
        }
        // println!("Processing group: {}", group);
        rv.extend(group.len().to_string().chars());
        rv.extend(group.chars().next());
    }

    rv
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut rv = input.trim().to_string();
    for _ in 0..50 {
        rv = look_and_say(&rv);
    }
    Ok(rv.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, look_and_say(input));
        Ok(())
    }

    #[rstest]
    #[case("1", "312211")]
    fn test_process_x5(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        let mut rv = input.trim().to_string();
        for _ in 0..5 {
            println!("b {rv}");
            rv = look_and_say(&rv);
            println!("a {rv}");
        }
        assert_eq!(expected, rv);
        Ok(())
    }
}
