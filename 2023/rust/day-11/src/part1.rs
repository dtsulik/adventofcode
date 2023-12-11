use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug)]
struct Vec2 {
    y: isize,
    x: isize,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (galaxies, row_check, column_check): (Vec<Vec2>, HashSet<isize>, HashSet<isize>) = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as isize;
            line.chars().enumerate().map(move |(x, t)| {
                let x = x as isize;
                if t == '#' {
                    (Some(Vec2 { x, y }), Some(y), Some(x))
                } else {
                    (None, None, None)
                }
            })
        })
        .fold(
            (Vec::new(), HashSet::new(), HashSet::new()),
            |(mut set3, mut set1, mut set2), (galaxy, set1_item, set2_item)| {
                if let Some(v) = set1_item {
                    set1.insert(v);
                }
                if let Some(v) = set2_item {
                    set2.insert(v);
                }
                if let Some(v) = galaxy {
                    set3.push(v);
                }
                (set3, set1, set2)
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
                        add += 1;
                    }
                }
            } else {
                for x in pair.x..g.x {
                    if !column_check.contains(&x) {
                        add += 1;
                    }
                }
            }

            if pair.y > g.y {
                for y in g.y..pair.y {
                    if !row_check.contains(&y) {
                        add += 1;
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

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
