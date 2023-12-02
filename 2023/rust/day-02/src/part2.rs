use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let rv = input.lines().map(|line| proccess_line(line)).sum::<i32>();

    Ok(rv.to_string())
}

#[tracing::instrument]
fn proccess_line(line: &str) -> i32 {
    let pts: Vec<&str> = line.split(":").collect();
    let games = pts[1].split(";").collect::<Vec<&str>>();
    let mut min_blue = 0;
    let mut min_green = 0;
    let mut min_red = 0;
    for g in games {
        for b in g.split(",").collect::<Vec<&str>>() {
            let pt = b.split(" ").collect::<Vec<&str>>();
            let n = pt[1].parse::<i32>().expect("must be num");
            match pt[2] {
                "blue" => {
                    if n > min_blue {
                        min_blue = n;
                    }
                }
                "red" => {
                    if n > min_red {
                        min_red = n;
                    }
                }
                "green" => {
                    if n > min_green {
                        min_green = n;
                    }
                }
                _ => {
                    panic!("this should never happen")
                }
            }
        }
    }
    min_red * min_blue * min_green
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}

