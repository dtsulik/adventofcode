use std::{cmp::Ordering, collections::HashMap, usize};

use crate::custom_error::AocError;

use nom::{
    branch::alt,
    character::complete::{self, alpha1, alphanumeric1, digit1, space1},
    multi::fold_many1,
    sequence::separated_pair,
    IResult, Parser,
};

const A: u32 = 14;
const K: u32 = 13;
const Q: u32 = 12;
const J: u32 = 11;
const T: u32 = 10;

fn parse_hand(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
    // fold_many1(
    //     alt((alpha1, digit1)),
    //     || 0 as u32,
    //     |acc: u32, x: &str| if x.starts_with("A") { acc + A } else { 0 },
    // )
    // .parse(input)
}

fn convert_char(c: char) -> u32 {
    match c {
        'A' => A,
        'K' => K,
        'Q' => Q,
        'J' => J,
        'T' => T,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("should never happen"),
    }
}

fn compare_hands(h1: &(&str, u32), h2: &(&str, u32)) -> Ordering {
    for (c1, c2) in h1.0.chars().zip(h2.0.chars()) {
        if c1 != c2 {
            let a = convert_char(c1);
            let b = convert_char(c2);
            return b.cmp(&a);
        };
    }
    Ordering::Equal
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut fives: Vec<(&str, u32)> = Vec::new();
    let mut four: Vec<(&str, u32)> = Vec::new();
    let mut three: Vec<(&str, u32)> = Vec::new();
    let mut twos: Vec<(&str, u32)> = Vec::new();
    let mut one: Vec<(&str, u32)> = Vec::new();
    let mut high: Vec<(&str, u32)> = Vec::new();
    let mut fullhouse: Vec<(&str, u32)> = Vec::new();

    let mut ranking = 0;

    input.lines().for_each(|line| {
        let (_, (hand, bid)) = separated_pair(parse_hand, space1, complete::u32)
            .parse(line)
            .unwrap();
        let mut cards = HashMap::new();
        for c in hand.chars() {
            *cards.entry(c).or_insert(0) += 1;
        }
        print!("hand: {hand} has ");
        let mut two_pair = false;
        let mut max_count = 0;
        for (_card, count) in cards {
            if max_count == 2 && count == 2 {
                two_pair = true;
            }
            if max_count == 3 && count == 2 || max_count == 2 && count == 3 {
                // right now I just want this tow work
                max_count = 7;
            }
            if count > max_count {
                max_count = count;
            }
            print!("'{}' x {}", _card, count);
        }
        println!();
        match max_count {
            5 => {
                println!("We have full house");
                fives.push((hand, bid));
            }
            4 => {
                println!("We have four of a kind");
                four.push((hand, bid));
            }
            3 => {
                println!("We have three of a kind");
                three.push((hand, bid));
            }
            2 => {
                if two_pair {
                    println!("We have pair of two of a kinds");
                    twos.push((hand, bid));
                } else {
                    println!("We have two of a kind");
                    one.push((hand, bid));
                }
            }
            1 => {
                println!("We have high card");
                high.push((hand, bid));
            }
            7 => {
                println!("We have full house");
                fullhouse.push((hand, bid));
            }
            _ => {
                panic!("yea nope")
            }
        }
        println!();
        ranking += 1;
    });

    let mut rv = 0;

    println!("{:?}", fives);
    fives.sort_by(compare_hands);
    println!("sorted fives {:?}", fives);
    for (h, bid) in fives {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("Sum total so far {rv}\n");

    println!("{:?}", four);
    four.sort_by(compare_hands);
    println!("sorted four {:?}", four);

    for (h, bid) in four {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("{:?}", fullhouse);
    fullhouse.sort_by(compare_hands);
    println!("sorted fullhouse {:?}", fullhouse);

    for (h, bid) in fullhouse {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    three.sort_by(compare_hands);
    println!("sorted three {:?}", three);

    for (h, bid) in three {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("{:?}", twos);
    twos.sort_by(compare_hands);
    println!("sorted two {:?}", twos);

    println!("Sum total so far {rv}\n");
    for (h, bid) in twos {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("{:?}", one);
    one.sort_by(compare_hands);
    println!("sorted one {:?}", one);
    println!("Sum total so far {rv}\n");

    for (h, bid) in one {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("{:?}", high);
    high.sort_by(compare_hands);
    println!("sorted high {:?}", high);

    println!("Sum total so far {rv}\n");
    for (h, bid) in high {
        println!("{h} has ranking {ranking} and {bid} {}", bid * ranking);
        rv += bid * ranking;
        ranking -= 1;
    }
    println!("Sum total so far {rv}\n");
    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
JJJJJ 219
99399 790
97999 604
93333 408
888A8 933";
        assert_eq!("29026", process(input)?);
        Ok(())
    }
}
