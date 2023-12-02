use crate::custom_error::AocError;

const MAX_RED: i32 = 12;
const MAX_BLUE: i32 = 14;
const MAX_GREEN: i32 = 13;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let rv = input.lines().map(|line| proccess_line(line)).sum::<i32>();

    Ok(rv.to_string())
}

#[tracing::instrument]
fn proccess_line(line: &str) -> i32 {
    let pts: Vec<&str> = line.split(":").collect();
    let game_id = pts[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .expect("must be num");
    let games = pts[1].split(";").collect::<Vec<&str>>();
    // let mut blue = 0;
    // let mut green = 0;
    // let mut red = 0;
    for g in games {
        for b in g.split(",").collect::<Vec<&str>>() {
            let pt = b.split(" ").collect::<Vec<&str>>();
            let n = pt[1].parse::<i32>().expect("must be num");
            match pt[2] {
                "blue" => {
                    if n > MAX_BLUE {
                        return 0;
                    }
                    // blue += n;
                }
                "red" => {
                    if n > MAX_RED {
                        return 0;
                    }
                    // red += n;
                }
                "green" => {
                    if n > MAX_GREEN {
                        return 0;
                    }
                    // green += n;
                }
                _ => {
                    panic!("this should never happen")
                }
            }
        }
    }
    game_id
    // if blue <= MAX_BLUE && green <= MAX_GREEN && red <= MAX_RED {
    //     println!("possible game id {game_id} had b{blue} r{red} g{green}");
    //     game_id
    // } else {
    //     println!("impossible game id {game_id} had b{blue} r{red} g{green}");
    //     0
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 4 red, 5 blue, 9 green; 7 green, 7 blue, 3 red; 16 red, 7 blue, 3 green; 11 green, 11 blue, 6 red; 12 red, 14 blue", 0)]
    #[case(
        "Game 2: 12 blue, 11 green, 3 red; 6 blue, 5 green, 7 red; 5 red, 11 blue; 2 blue, 8 green",
        2
    )]
    #[case("Game 3: 8 blue, 5 green, 2 red; 5 blue, 5 green, 7 red; 7 blue, 1 green, 7 red; 8 green, 14 blue, 7 red; 8 green, 14 blue; 8 blue, 2 green, 8 red", 3)]
    #[case(
        "Game 4: 3 red, 14 blue, 15 green; 1 red, 11 green, 14 blue; 14 green, 17 blue",
        0
    )]
    #[case("Game 5: 11 green, 2 red, 10 blue; 16 green, 8 blue; 2 blue, 6 green, 1 red; 14 blue, 2 red, 16 green; 3 blue, 18 green; 1 red, 10 blue, 3 green", 0)]
    #[case(
        "Game 6: 2 green, 5 red, 17 blue; 12 green, 13 blue, 6 red; 8 red, 9 blue, 7 green",
        0
    )]
    #[case(
        "Game 7: 2 blue, 18 green; 4 green, 1 red, 1 blue; 4 blue, 19 green",
        0
    )]
    #[case("Game 8: 6 green, 7 blue; 9 green, 6 blue; 7 blue, 1 red, 3 green", 8)]
    #[case("Game 9: 4 blue, 12 green, 3 red; 4 green, 3 blue, 3 red; 3 green, 2 red, 3 blue; 1 green, 2 red, 3 blue; 15 red, 10 green, 4 blue; 3 blue, 1 red, 6 green", 0)]
    #[case(
        "Game 10: 11 blue, 6 green, 6 red; 15 green, 1 blue; 1 red, 6 blue, 4 green",
        0
    )]
    #[case(
        "Game 11: 9 blue, 1 red, 6 green; 6 red, 1 green; 10 blue, 3 green, 6 red",
        11
    )]
    #[case(
        "Game 12: 1 blue, 10 red, 1 green; 4 blue, 4 red; 8 red, 3 blue, 1 green; 3 red, 2 green",
        12
    )]
    #[case("Game 13: 3 red, 11 green, 18 blue; 11 green, 1 red, 3 blue; 12 blue, 5 red, 2 green; 16 blue, 8 red, 5 green; 8 red, 12 blue, 19 green; 17 blue, 4 green, 6 red", 0)]
    #[case(
        "Game 14: 8 red, 4 blue; 1 green, 2 blue, 13 red; 1 green, 1 blue, 17 red; 1 green, 13 red",
        0
    )]
    #[case("Game 15: 4 red, 3 blue, 6 green; 4 blue, 3 red, 3 green; 3 green, 6 red, 3 blue; 6 red, 5 blue, 2 green; 6 green, 1 blue; 4 green, 3 red, 2 blue", 15)]
    #[case("Game 16: 11 green; 3 green, 1 blue, 1 red; 12 green, 3 blue, 1 red; 1 red, 1 green; 1 red, 3 blue; 2 green, 1 blue, 1 red", 16)]
    #[case("Game 17: 12 red, 14 blue, 10 green; 2 red, 6 green, 6 blue; 10 blue, 2 green, 3 red; 1 red, 13 blue, 2 green; 9 green, 16 red, 9 blue", 0)]
    #[case("Game 18: 15 red, 8 blue; 16 red, 12 blue; 5 blue, 4 green, 6 red; 8 red, 4 green, 3 blue; 7 red, 5 blue, 2 green; 1 blue, 2 green, 14 red", 0)]
    #[case("Game 19: 3 red, 13 blue, 2 green; 8 red, 14 blue; 9 blue, 3 green; 9 blue, 1 green, 7 red; 8 red, 1 green; 8 red, 14 blue, 2 green", 19)]
    #[case("Game 20: 6 green, 10 blue, 5 red; 8 green, 9 blue, 7 red; 2 red, 2 green, 7 blue; 7 red, 16 green, 12 blue; 15 green, 3 red; 12 green, 3 red, 6 blue", 0)]
    fn line_test(#[case] line: &str, #[case] expected: i32) {
        assert_eq!(expected, proccess_line(line))
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
