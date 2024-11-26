use std::{collections::HashMap, ops::Add};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0, space1},
    sequence::preceded,
    IResult,
};

use crate::custom_error::AocError;

// TODO in this case it def is a way to avoid a map
#[tracing::instrument]
fn overlap_area(rect1: ((i32, i32), (i32, i32)), rect2: ((i32, i32), (i32, i32))) -> i32 {
    let ((x1, y1), (j1, k1)) = rect1;
    let ((x2, y2), (j2, k2)) = rect2;

    let left = x1.max(x2);
    let right = j1.min(j2);
    let top = y1.max(y2);
    let bottom = k1.min(k2);

    if left < right && top < bottom {
        (right - left + 1) * (bottom - top + 1)
    } else {
        0
    }
}

fn parse_coords(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, x_str) = digit1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y_str) = digit1(input)?;
    let x = x_str.parse::<i32>().unwrap();
    let y = y_str.parse::<i32>().unwrap();
    Ok((input, (x, y)))
}

fn parse_line(input: &str) -> IResult<&str, (String, (i32, i32), (i32, i32))> {
    let (input, action) = preceded(
        space0,
        alt((tag("turn on"), tag("turn off"), tag("toggle"))),
    )(input)?;

    let (input, _) = space1(input)?;
    let (input, first_coords) = parse_coords(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("through")(input)?;
    let (input, _) = space1(input)?;
    let (input, second_coords) = parse_coords(input)?;

    Ok((input, (action.to_string(), first_coords, second_coords)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    input.lines().for_each(|l| {
        let (_, (action, (x, y), (j, k))) = parse_line(l).unwrap();

        match action.as_str() {
            "turn on" => {
                let x_range = x..=j;
                let y_range = y..=k;

                for x in x_range {
                    for y in y_range.clone() {
                        let e = grid.entry((x, y)).or_insert(0);
                        *e = e.add(1);
                    }
                }
            }
            "turn off" => {
                let x_range = x..=j;
                let y_range = y..=k;

                for x in x_range {
                    for y in y_range.clone() {
                        let e = grid.entry((x, y)).or_insert(0);
                        *e = e.saturating_sub(1);
                    }
                }
            }
            "toggle" => {
                let x_range = x..=j;
                let y_range = y..=k;

                for x in x_range {
                    for y in y_range.clone() {
                        let e = grid.entry((x, y)).or_insert(0);
                        *e = e.add(2);
                    }
                }
            }
            _ => {}
        }
    });

    Ok(grid.values().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(((0,0),(4,4)),((1,1),(2,2)), 4)]
    fn test_overlap(
        #[case] input1: ((i32, i32), (i32, i32)),
        #[case] input2: ((i32, i32), (i32, i32)),
        #[case] expected: i32,
    ) -> miette::Result<()> {
        assert_eq!(expected, overlap_area(input1, input2));
        Ok(())
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", "1")]
    #[case("toggle 0,0 through 999,999", "2000000")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
