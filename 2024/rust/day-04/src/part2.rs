use crate::custom_error::AocError;

// use crate::part1::PrettyPrint;

macro_rules! top_left {
    ($matrix:expr, $i:expr, $j:expr) => {
        $matrix[$i - 1][$j - 1]
    };
}

macro_rules! top_right {
    ($matrix:expr, $i:expr, $j:expr) => {
        $matrix[$i - 1][$j + 1]
    };
}

macro_rules! bottom_left {
    ($matrix:expr, $i:expr, $j:expr) => {
        $matrix[$i + 1][$j - 1]
    };
}

macro_rules! bottom_right {
    ($matrix:expr, $i:expr, $j:expr) => {
        $matrix[$i + 1][$j + 1]
    };
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // println!("{}\n", matrix.pretty_print());

    let mut sum = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] == 'A' {
                // println!("Debugging at ({}, {}): top-left={}, top-right={}, bottom-left={}, bottom-right={}",
                //     i, j,
                //     matrix[i - 1][j - 1],
                //     matrix[i - 1][j + 1],
                //     matrix[i + 1][j - 1],
                //     matrix[i + 1][j + 1],
                // );
                // look around
                if ((top_left!(matrix, i, j) == 'M' && bottom_right!(matrix, i, j) == 'S')
                    || (top_left!(matrix, i, j) == 'S') && bottom_right!(matrix, i, j) == 'M')
                    && ((top_right!(matrix, i, j) == 'M' && bottom_left!(matrix, i, j) == 'S')
                        || (top_right!(matrix, i, j) == 'S' && bottom_left!(matrix, i, j) == 'M'))
                {
                    // println!("\t Match at ({}, {}): top-left={}, top-right={}, bottom-left={}, bottom-right={}",
                    //     i, j,
                    //     matrix[i - 1][j - 1],
                    //     matrix[i - 1][j + 1],
                    //     matrix[i + 1][j - 1],
                    //     matrix[i + 1][j + 1],
                    // );
                    sum += 1;
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        "9"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
