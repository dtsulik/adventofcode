use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let dir_map = |d| match d {
        '>' => directions[0],
        '<' => directions[1],
        '^' => directions[2],
        'v' => directions[3],
        _ => (0, 0),
    };
    grid.insert((0, 0));

    let mut santa_or_robo = 1;
    input.chars().for_each(|d| {
        let dir = dir_map(d);
        if santa_or_robo == 1 {
            santa_pos = (santa_pos.0 + dir.0, santa_pos.1 + dir.1);
            grid.insert(santa_pos);
            santa_or_robo = 0;
        } else {
            robo_pos = (robo_pos.0 + dir.0, robo_pos.1 + dir.1);
            grid.insert(robo_pos);
            santa_or_robo = 1;
        }
    });

    Ok(grid.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("^v", "3")]
    #[case("^>v<", "3")]
    #[case("^v^v^v^v^v", "11")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
