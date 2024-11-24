use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|l| {
            let dimensions = l.splitn(3, 'x').collect::<Vec<&str>>();
            if let [l, w, h] = &dimensions[..] {
                if let (Ok(l), Ok(w), Ok(h)) =
                    (l.parse::<i32>(), w.parse::<i32>(), h.parse::<i32>())
                {
                    2 * (l * w + w * h + h * l) + (l * w).min(w * h).min(h * l)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", "58")]
    #[case("1x1x10", "43")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
