use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut count = 0;
        let dirs: [(isize, isize); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1)
        ];
        let target = "XMAS";
        for i in 0..lines.len() {
            for j in 0..lines[0].len() {
                'dir: for (dx, dy) in &dirs {
                    for (d, c) in target.chars().enumerate() {
                        let x = i as isize + d as isize * dx;
                        let y = j as isize + d as isize * dy;
                        if x < 0 || x >= lines.len() as isize || y < 0 || y >= lines[0].len() as isize {
                            continue 'dir;
                        }
                        if lines[x as usize].as_bytes()[y as usize] != c as u8 {
                            continue 'dir;
                        }
                    }
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut count = 0;
        let dirs = [
            ((1, 1), (-1, -1)),
            ((1, -1), (-1, 1)),
        ];
        for i in 1..(lines.len() - 1) {
            'cell: for j in 1..(lines[0].len() - 1) {
                if lines[i].as_bytes()[j] != 'A' as u8 {
                    continue 'cell;
                }
                for ((dx1, dy1), (dx2, dy2)) in &dirs {
                    let x1 = i as isize + dx1;
                    let y1 = j as isize + dy1;
                    let x2 = i as isize + dx2;
                    let y2 = j as isize + dy2;
                    let c1 = lines[x1 as usize].as_bytes()[y1 as usize] as char;
                    let c2 = lines[x2 as usize].as_bytes()[y2 as usize] as char;
                    match (c1, c2) {
                        ('M', 'S') | ('S', 'M') => {}
                        _ => { continue 'cell; }
                    }
                }
                count += 1;
            }
        }
        Ok(count)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
