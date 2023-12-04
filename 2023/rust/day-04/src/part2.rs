use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
//     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
//
    let mut copy_map: Vec<i64> = vec![0; input.len() + 1];

    let rv: i64 = input
        .lines()
        .map(|line| {
            let (id, card) = line.split_once(':').unwrap();
            let (_, i) = id.split_once(' ').unwrap();
            // println!("Working on card {} {line}", i.trim());
            let i = i.trim().parse::<usize>().unwrap();
            let (winning, owned) = card.split_once('|').unwrap();

            let winning_num: Vec<i32> = winning
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let owned_num: Vec<i32> = owned
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let n: i64 = owned_num.iter().fold(0, |acc, o| {
                if winning_num.contains(o) {
                    acc + 1
                } else {
                    acc
                }
            });

            // println!("Card {i} won {n} times. For each of those wins we add:");

            for x in 0..n {
                let idx = i + 1 + x as usize;
                // print!("Current copy for {idx}: {}", copy_map[idx]);
                copy_map[idx] = copy_map[idx] + 1 + copy_map[i];
                // println!(" + 1 = {}", copy_map[idx]);
            }
            // println!("\nTotal cards of {i}: {}", copy_map[i] + 1);
            // println!("==========================================");
            copy_map[i] + 1
        })
        .sum();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}

