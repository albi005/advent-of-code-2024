use std::collections::HashSet;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rows: Vec<_> = reader
            .lines()
            .flatten()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect();
        let mut curr = rows
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, c)| {
                    if c == &b'^' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let mut dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
        let mut dir = dirs.next().unwrap();

        loop {
            let next = (curr.0 + dir.0, curr.1 + dir.1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= rows[0].len() as isize
                || next.1 >= rows.len() as isize
            {
                break;
            }
            let c = &mut rows[next.1 as usize][next.0 as usize];
            if c != &b'#' {
                curr = next;
                *c = b'X';
            }
            else {
                dir = dirs.next().unwrap();
            }
        };
        
        Ok(rows.iter()
            .map(|row| row.iter().filter(|c| c == &&b'X').count())
            .sum())
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        enum Dir {
            Up, Down, Left, Right
        }
        
        let rows: Vec<_> = reader
            .lines()
            .flatten()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect();
        let start = rows
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, c)| {
                    if c == &b'^' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        let mut count = 0;

        for replaced_y in 0..rows.len() {
            for replaced_x in 0..rows[0].len() {
                if rows[replaced_y][replaced_x] != b'.' {
                    continue;
                }
                let mut dirs = [(0, -1, Dir::Up), (1, 0, Dir::Right), (0, 1, Dir::Down), (-1, 0, Dir::Left)].iter().cycle();
                let mut dir = dirs.next().unwrap();
                let mut curr = start;
                let mut visited: Vec<Vec<HashSet<Dir>>> = (0..rows.len()).map(|_| (0..rows[0].len()).map(|_| HashSet::new()).collect()).collect();
                loop {
                    let next = (curr.0 + dir.0, curr.1 + dir.1);
                    if next.0 < 0
                        || next.1 < 0
                        || next.0 >= rows[0].len() as isize
                        || next.1 >= rows.len() as isize
                    {
                        break;
                    }
                    let x = next.0 as usize;
                    let y = next.1 as usize;
                    let mut next_c = rows[y][x];
                    if x == replaced_x && y == replaced_y {
                        next_c = b'#';
                    }
                    if next_c != b'#' {
                        curr = next;
                    }
                    else {
                        dir = dirs.next().unwrap();
                    }
                    let visited = &mut visited[y][x];
                    let dir_dir = dir.2;
                    if visited.contains(&dir_dir) {
                        count += 1;
                        break;
                    }
                    visited.insert(dir_dir);
                };
            }
        }

        Ok(count)
    }
    
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
