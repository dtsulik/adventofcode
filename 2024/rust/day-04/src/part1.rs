use crate::custom_error::AocError;
use std::fmt;

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl<T: fmt::Debug> PrettyPrint for Vec<Vec<T>> {
    fn pretty_print(&self) -> String {
        self.iter()
            .map(|row| format!("{:?}", row))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn rotate(matrix: &[Vec<char>], degree: i32) -> Vec<Vec<char>> {
    let n = matrix.len();
    let m = matrix[0].len();

    match degree {
        0 => matrix.to_vec(),
        45 => {
            let mut rotated: Vec<Vec<char>> = vec![vec![]; n + m - 1];
            for i in 0..n {
                for j in 0..m {
                    rotated[i + j].push(matrix[i][j]);
                }
            }
            for diag in &mut rotated {
                diag.reverse();
            }
            rotated
        }
        90 => {
            let mut rotated: Vec<Vec<char>> = vec![vec![' '; n]; m];
            for i in 0..n {
                for j in 0..m {
                    rotated[j][n - i - 1] = matrix[i][j];
                }
            }
            rotated
        }
        135 => {
            let mut rotated: Vec<Vec<char>> = vec![vec![]; n + m - 1];
            for i in 0..n {
                for j in 0..m {
                    rotated[i + (m - 1 - j)].push(matrix[i][j]);
                }
            }
            for diag in &mut rotated {
                diag.reverse();
            }
            rotated
        }
        _ => panic!(
            "Unsupported degree: {}. Only 0, 45, 90, and 135 are supported.",
            degree
        ),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // println!("{}\n", matrix.pretty_print());

    let sum: i32 = [0, 45, 90, 135]
        .iter()
        .map(|&d| {
            let rot = rotate(&matrix, d);
            // println!("degree {}:\n{}\n", d, rot.pretty_print());
            rot.iter()
                .map(|l| {
                    l.windows(4)
                        .map(|w| if w == ['X', 'M', 'A', 'S'] || w == ['S','A','M','X']  { 1 } else { 0 })
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum();

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
        "18"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
