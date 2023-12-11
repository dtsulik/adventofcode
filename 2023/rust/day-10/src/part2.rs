use crate::custom_error::AocError;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::collections::HashMap;

use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, ExecutableCommand};

use std::io::{self, Write};
use std::{thread, time};

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Clone, Copy)]
struct Node {
    y: isize,
    x: isize,
}

const NORTH: Node = Node { x: 0, y: -1 };
const SOUTH: Node = Node { x: 0, y: 1 };
const EAST: Node = Node { x: 1, y: 0 };
const WEST: Node = Node { x: -1, y: 0 };

fn cleanup() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    stdout.execute(cursor::Show)?;

    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn print_maze(maze: &HashMap<Node, char>, clear: bool) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    // Sort the nodes by their coordinates
    let mut nodes: Vec<_> = maze.keys().collect();
    nodes.sort();

    if clear {
        stdout.execute(cursor::MoveTo(0, 0))?; // Move cursor to start
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::Hide)?;
    }

    let mut current_y = 0;

    for node in nodes {
        if node.y > current_y {
            // Move to the next line if y coordinate changesa
            writeln!(stdout)?;
            let (_, y) = cursor::position().unwrap();
            stdout.execute(cursor::MoveTo(0, y + node.y as u16))?;
            current_y = node.y;
        }

        // Print the character at the current node
        write!(stdout, "{}", maze[node])?;
    }

    writeln!(stdout)?;
    stdout.flush()?;
    terminal::disable_raw_mode()?;

    thread::sleep(time::Duration::from_millis(500));

    Ok(())
}

fn get_direction(node: char, direction: Node) -> Node {
    match node {
        '-' => direction,
        '|' => direction,
        _ => match direction {
            NORTH => {
                return match node {
                    '7' => WEST,
                    'F' => EAST,
                    _ => panic!("Came from north to node {node}"),
                };
            }
            SOUTH => {
                return match node {
                    'J' => WEST,
                    'L' => EAST,
                    _ => panic!("Came from south to node {node}"),
                };
            }
            WEST => {
                return match node {
                    'L' => NORTH,
                    'F' => SOUTH,
                    _ => panic!("Came from west to node {node}"),
                };
            }
            EAST => {
                return match node {
                    'J' => NORTH,
                    '7' => SOUTH,
                    _ => panic!("Came from east to node {node}"),
                };
            }
            _ => {
                panic!("Should always have valid direction");
            }
        },
    }
}

fn is_neighbor_valid(nei: char, direction: Node) -> bool {
    match direction {
        SOUTH => {
            return match nei {
                'J' => true,
                '|' => true,
                'L' => true,
                _ => false,
            };
        }
        NORTH => {
            return match nei {
                '7' => true,
                '|' => true,
                'F' => true,
                _ => false,
            };
        }
        EAST => {
            return match nei {
                'J' => true,
                '7' => true,
                '-' => true,
                _ => false,
            };
        }
        WEST => {
            return match nei {
                'L' => true,
                'F' => true,
                '-' => true,
                _ => false,
            };
        }
        _ => {
            panic!("Should always have valid direction");
        }
    }
}

fn walk(maze: &mut HashMap<Node, char>, node: Node, from: Node, seen: &mut HashMap<Node, char>) {
    let kv = maze.get_key_value(&node);
    match kv {
        Some((&n, &n_type)) => {
            let to = get_direction(n_type, from);

            let nein = Node {
                x: n.x + to.x,
                y: n.y + to.y,
            };

            let nei = maze.get(&nein);
            match nei {
                Some(ne) => {
                    if *ne != 'S' {
                        seen.insert(n, '*');
                        walk(maze, nein, to, seen);
                    } else {
                        seen.insert(n, '*');
                    }
                }
                None => {}
            }
        }
        None => {
            panic!("no");
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    //     let input = "...........
    // .S-------7.
    // .|F-----7|.
    // .||.....||.
    // .||.....||.
    // .|L-7.F-J|.
    // .|..|.|..|.
    // .L--J.L--J.
    // ...........";

    let mut maze: HashMap<Node, char> = HashMap::new();
    let mut start = Node { x: 0, y: 0 };

    let directions = vec![NORTH, SOUTH, WEST, EAST];
    input.lines().enumerate().for_each(|(y, l)| {
        let y = (y + 1) as isize;
        l.chars().enumerate().for_each(|(x, n)| {
            let x = (x + 1) as isize;
            maze.insert(Node { x, y }, n);
            if n == 'S' {
                start = Node { x, y };
            }
        });
    });

    let mut seen = maze.clone();
    let kv = maze.get_key_value(&start);
    match kv {
        Some((&n, &nt)) => {
            for direction in directions {
                let nx = n.x + direction.x;
                let ny = n.y + direction.y;

                if nx < 0 || ny < 0 {
                    continue;
                }
                let nein = Node { x: nx, y: ny };
                let nei = maze.get(&nein);
                match nei {
                    Some(ne) => {
                        if is_neighbor_valid(*ne, direction) {
                            seen.insert(start, '*');
                            walk(&mut maze.clone(), nein, direction, &mut seen);
                            break;
                        }
                    }
                    None => {}
                }
            }
        }
        None => {
            panic!("no");
        }
    }

    // let _ = print_maze(&maze, false);
    // let _ = print_maze(&seen, false);

    let rv: i32 = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            let mut count = 0;
            let mut crossed = 0;
            let y = (y + 1) as isize;
            l.chars().enumerate().for_each(|(x, c)| {
                let x = (x + 1) as isize;
                let n = Node { x, y };
                // check if its part of the path
                let check = seen[&n];
                // println!("Crossing {y} current node {c} check {check}");
                if check == '*' {
                    match c {
                        '.' => {
                            if crossed % 2 != 0 {
                                count += 1;
                            }
                        }
                        // check this
                        'S' => {
                            crossed += 1;
                        }
                        '|' => {
                            crossed += 1;
                        }
                        'F' => {
                            crossed += 1;
                        }
                        '7' => {
                            crossed += 1;
                        }
                        'L' => {}
                        'J' => {}
                        '-' => {} // since we are checking from west to east no need to check
                        // horizontal lines
                        _ => {
                            panic!("no");
                        }
                    }
                } else {
                    if crossed % 2 != 0 {
                        count += 1;
                    }
                }
            });
            count
        })
        .sum();

    let _ = cleanup();

    Ok(rv.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        "4"
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        "8"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
