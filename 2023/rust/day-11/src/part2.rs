use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug)]
struct Vec2 {
    y: isize,
    x: isize,
}

#[tracing::instrument]
pub fn process(input: &str, expantion: isize) -> miette::Result<String, AocError> {
    let expantion = expantion -1;
//     let input = "...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....";

    let (_universe, galaxies, row_check, column_check): (
        HashMap<Vec2, char>,
        Vec<Vec2>,
        HashSet<isize>,
        HashSet<isize>,
    ) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as isize;
            line.chars().enumerate().map(move |(x, t)| {
                let x = x as isize;
                if t == '#' {
                    (Vec2 { x, y }, Some(Vec2 { x, y }), t, Some(y), Some(x))
                } else {
                    (Vec2 { x, y }, None, t, None, None)
                }
            })
        })
        .fold(
            (HashMap::new(), Vec::new(), HashSet::new(), HashSet::new()),
            |(mut map, mut set3, mut set1, mut set2),
             (vec2, galaxy, node, set1_item, set2_item)| {
                map.insert(vec2, node);
                if let Some(v) = set1_item {
                    set1.insert(v);
                }
                if let Some(v) = set2_item {
                    set2.insert(v);
                }
                if let Some(v) = galaxy {
                    set3.push(v);
                }
                (map, set3, set1, set2)
            },
        );

    let mut rv = 0;

    let mut mp = itertools::multipeek(galaxies.iter());
    while let Some(g) = mp.next() {
        while let Some(pair) = mp.peek() {
            let d2 = (pair.x - g.x).abs() + (pair.y - g.y).abs();
            let mut add = 0;
            if pair.x > g.x {
                for x in g.x..pair.x {
                    if !column_check.contains(&x) {
                        add += expantion;
                    }
                }
            } else {
                for x in pair.x..g.x {
                    if !column_check.contains(&x) {
                        add += expantion;
                    }
                }
            }

            if pair.y > g.y {
                for y in g.y..pair.y {
                    if !row_check.contains(&y) {
                        add += expantion;
                    }
                }
            }
            rv += d2 + add;
        }
    }

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        1,
        "374"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        10,
        "1030"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        100,
        "8410"
    )]
    fn test_process(
        #[case] input: &str,
        #[case] expantion: isize,
        #[case] expected: &str,
    ) -> miette::Result<()> {
        assert_eq!(expected, process(input, expantion)?);
        Ok(())
    }
}
