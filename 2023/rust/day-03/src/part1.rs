use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let line_len = input.lines().next().map_or(0, |line| line.len() + 1);
    let mut rv = 0;
    let mut seen: HashMap<usize, bool> = HashMap::new();

    let directions: [isize; 8] = [
        -1,
        1,
        -(line_len as isize),
        line_len as isize,
        -(line_len as isize) - 1,
        -(line_len as isize) + 1,
        line_len as isize - 1,
        line_len as isize + 1,
    ];

    for (i, c) in input.char_indices() {
        if i < line_len || input.len() - i < line_len {
            //this is last or first line skip.
            continue;
        }
        if c == '.' || seen.contains_key(&i) || c == '\n' {
            continue;
        }

        if is_symbol(c) {
            // debug_loc(input, i, line_len);
            // debug_loc2(input, i, line_len, 6);
            for &offset in &directions {
                let p = (i as isize + offset) as usize;
                // println!("looking fo rnum at {p}");
                if input.chars().nth(p).unwrap().is_ascii_digit() {
                    if let Some(num) =
                        // check_and_get_number(i as isize + offset, line_len, &input, &mut seen)
                        get_num_at(input, p, &mut seen)
                    {
                        // println!("Found {num}");
                        rv += num;
                    }
                }
            }
        }
    }

    Ok(rv.to_string())
}

fn get_num_at(input: &str, i: usize, seen: &mut HashMap<usize, bool>) -> Option<u32> {
    if seen.contains_key(&i) {
        return None;
    }
    let mut s = String::new();
    let dot = if !input.chars().nth(i + 1).unwrap().is_ascii_digit() {
        i + 1
    } else if !input.chars().nth(i + 2).unwrap().is_ascii_digit() {
        i + 2
    } else {
        i + 3
    };

    if input.chars().nth(dot - 3).unwrap().is_ascii_digit() && !seen.contains_key(&(dot - 3)) {
        s.push(input.chars().nth(dot - 3).unwrap());
        seen.insert(dot - 3, true);
        // println!("digit at {}", dot - 3);
    }
    if input.chars().nth(dot - 2).unwrap().is_ascii_digit() && !seen.contains_key(&(dot - 2)) {
        s.push(input.chars().nth(dot - 2).unwrap());
        seen.insert(dot - 2, true);
        // println!("digit at {}", dot - 2);
    }
    if input.chars().nth(dot - 1).unwrap().is_ascii_digit() && !seen.contains_key(&(dot - 1)) {
        s.push(input.chars().nth(dot - 1).unwrap());
        seen.insert(dot - 1, true);
        // println!("digit at {}", dot - 1);
    }
    // println!("Is this a num? {s}");
    if s.len() == 0 {
        return None;
    }
    let n = s.parse::<u32>().unwrap();
    Some(n)
}

fn is_symbol(c: char) -> bool {
    matches!(
        c,
        '*' | '/' | '$' | '=' | '%' | '+' | '-' | '#' | '@' | '\\' | '&'
    )
}

fn debug_loc2(input: &str, i: usize, line_len: usize, range: isize) {
    // Convert input to a vector of chars for easier indexing
    let chars: Vec<char> = input.chars().collect();
    let input_len = chars.len();

    // Calculate the start and end positions for the range
    let start_row = i / line_len as usize;
    let start_col = i % line_len;

    println!("Debugging around index {}: ", i);
    for row in start_row as isize - range..=start_row as isize + range {
        for col in start_col as isize - range..=start_col as isize + range {
            // Calculate the index
            let idx = row * line_len as isize + col;
            if idx >= 0
                && (idx as usize) < input_len
                && row >= 0
                && (row as usize * line_len) < input_len
            {
                print!("{}", chars[idx as usize]);
            }
        }
        println!();
    }
    println!();
}

fn debug_loc(input: &str, i: usize, line_len: usize) {
    let c = input.chars().nth(i).unwrap();
    let prev_char = input.chars().nth(i - 1).unwrap();
    let next_char = input.chars().nth(i + 1).unwrap();
    let above_char = input.chars().nth(i - line_len).unwrap();
    let below_char = input.chars().nth(i + line_len).unwrap();
    // need to check if they are on edges, mayb converting to 2d arr would be beter
    let above_prev_char = input.chars().nth(i - line_len - 1).unwrap();
    let above_next_char = input.chars().nth(i - line_len + 1).unwrap();
    let below_prev_char = input.chars().nth(i + line_len - 1).unwrap();
    let below_next_char = input.chars().nth(i + line_len + 1).unwrap();
    println!(
        "Found symbol {c} chars around it:
{above_prev_char}{above_char}{above_next_char}
{prev_char}{c}{next_char}
{below_prev_char}{below_char}{below_next_char}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
