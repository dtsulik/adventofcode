use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut num_left: Vec<u32> = Vec::new();
    let mut num_right: Vec<u32> = Vec::new();
    input.lines().for_each(|l| {
        let (left, right) = l.trim().split_once("   ").expect("wth");
        num_left.push(left.parse().unwrap());
        num_right.push(right.parse().unwrap());
    });

    num_left.sort();
    num_right.sort();

    Ok(num_left.iter().zip(num_right).map(|(&l, r)| {
        if l > r{
            l - r
        } else {
            r - l
        }
    }).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "3   4
4   3
2   5
1   3
3   9
3   3",
        "11"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
