use crate::custom_error::AocError;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let mut it = l.chars().peekable();

            let mut vcount = 0;
            let mut has_double = false;
            let mut has_naughty = false;
            while let Some(curr) = it.next() {
                if VOWELS.contains(&curr) {
                    vcount += 1;
                }
                if let Some(next) = it.peek() {
                    if curr == *next {
                        has_double = true
                    }
                    let tmp = format!("{curr}{next}");
                    if tmp == "ab" || tmp == "cd" || tmp == "pq" || tmp == "xy" {
                        has_naughty = true
                    }
                }
            }

            if vcount >= 3 && has_double && !has_naughty {
                1
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", "1")]
    #[case("aaa", "1")]
    #[case("jchzalrnumimnmhp", "0")]
    #[case("haegwjzuvuyypxyu", "0")]
    #[case("dvszwmarrgswjxmb", "0")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
