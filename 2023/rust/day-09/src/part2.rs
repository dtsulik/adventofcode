use itertools::Itertools;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult, Parser,
};

// use rayon::{prelude, str::ParallelString};

use crate::custom_error::AocError;

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64).parse(input)
}

fn rec_collect(seqs: &mut Vec<Vec<i64>>, curr: Vec<i64>) {
    // println!("collecting: {:?}", curr);
    seqs.push(curr.clone());
    let next_seq: Vec<i64> = curr
        .iter()
        .tuple_windows()
        .map(|(&curr, &next)| next - curr)
        .collect();

    if !next_seq.iter().all(|s| *s == 0) {
        rec_collect(seqs, next_seq);
    }
}
fn rec_process(seqs: &mut Vec<Vec<i64>>) {
    if seqs.len() == 1 {
        return;
    }

    // println!("Seqs: {:?}", seqs);

    if let Some(last) = seqs.pop() {
        // Get the last number of the last sequence (now removed)
        if let Some(last_num) = last.first() {
            // Check the last sequence in 'seqs' and calculate the difference
            if let Some(next) = seqs.last_mut() {
                if let Some(next_num) = next.first() {
                    // println!(
                    //     "adding next: {} + {} = {}",
                    //     next_num,
                    //     last_num,
                    //     next_num + last_num
                    // );
                    let diff = next_num - last_num;
                    // Now push the difference to 'next'
                    next.insert(0, diff);
                }
            }
        }
    }

    rec_process(seqs);
}

fn process_seq(seq: Vec<i64>) -> i64 {
    let mut seqs = Vec::new();
    rec_collect(&mut seqs, seq);
    // println!("seqs: {:?}", seqs);
    rec_process(&mut seqs);
    let rv = seqs[0].first().unwrap().clone();
    // println!("updated: {:?}\n", seqs);
    rv
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let input = "0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45";

    let rv = input
        .lines()
        .map(|line| {
            let (_, seq) = parse_line(line).expect("must be line of numbers");
            process_seq(seq)
        })
        .sum::<i64>();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}



