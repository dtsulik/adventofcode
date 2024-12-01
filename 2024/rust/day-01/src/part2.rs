use std::collections::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut num_left: Vec<u32> = Vec::new();
    let mut appears: HashMap<u32, u32> = HashMap::new();
    input.lines().for_each(|l| {
        let (left, right) = l.trim().split_once("   ").expect("wth");
        num_left.push(left.parse().unwrap());
        appears.entry(right.parse().unwrap()).and_modify(|e| {*e+=1}).or_insert(1);
    });


    Ok(num_left.iter().map(|l| {
        l * appears.get(l).or(Some(&0)).unwrap()
    }).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3   4
4   3
2   5
1   3
3   9
3   3", "31")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}

