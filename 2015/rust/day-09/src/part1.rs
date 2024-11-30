use std::collections::HashMap;
use std::cmp::min;

use crate::custom_error::AocError;
use crate::parser::parse_line;

fn travel(mask: i32, pos: usize, n: usize, adj_matrix: &Vec<Vec<i32>>) -> i32 {
    if mask == (1 << n) - 1 {
        return 0;
    }

    let mut rv = i32::MAX;

    for i in 0..n {
        if mask & (1 << i) == 0 {
            rv = min(
                rv,
                adj_matrix[pos][i] + travel(mask | (1 << i), i, n, adj_matrix),
            );
        }
    }
    rv
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    let mut edges: Vec<(&str, &str, i32)> = Vec::new();
    for line in input.lines() {
        let (_, (len, (src, dst))) = parse_line(line).unwrap();

        edges.push((src, dst, len));

        if !nodes.contains_key(src) {
            nodes.insert(src, nodes.len());
        }
        if !nodes.contains_key(dst) {
            nodes.insert(dst, nodes.len());
        }
    }

    let size = nodes.len();

    let mut adj_matrix = vec![vec![0; size]; size];

    for &(src, dst, len) in &edges {
        let src_idx = nodes[src];
        let dst_idx = nodes[dst];
        adj_matrix[src_idx][dst_idx] = len;
        adj_matrix[dst_idx][src_idx] = len;
    }

    // for row in &adj_matrix {
    //     println!("{:?}", row);
    // }

    let min = travel(1, 0, adj_matrix.len(), &adj_matrix);

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        "605"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
