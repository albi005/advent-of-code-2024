use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::Write;
use std::io::{stdout, BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#........#
#........#
#........#
#........#
#...O....#
#..OO....#
#...O@...#
#........#
##########

<>^^<^<v
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let mut parts = input.split("\n\n");
        let mut map: Vec<Vec<u8>> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.bytes().collect())
            .collect();
        let cmds: Vec<u8> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.bytes())
            .flatten()
            .collect();
        let mut pos = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                if let Some(x) = line.iter().position(|c| *c == b'@') {
                    Some((x, y))
                } else {
                    None
                }
            })
            .unwrap();
        for cmd in cmds {
            let dir = match cmd {
                b'<' => (-1, 0),
                b'^' => (0, -1),
                b'>' => (1, 0),
                b'v' => (0, 1),
                _ => panic!(),
            };
            fn mov(x: usize, y: usize, dir: (isize, isize), map: &mut Vec<Vec<u8>>) -> bool {
                let c = map[y][x];
                if c == b'.' {
                    return true;
                }
                if c == b'#' {
                    return false;
                }
                let end_x = (x as isize + dir.0) as usize;
                let end_y = (y as isize + dir.1) as usize;
                if mov(end_x, end_y, dir, map) {
                    map[end_y][end_x] = map[y][x];
                    return true;
                }
                false
            }
            if mov(pos.0, pos.1, dir, &mut map) {
                map[pos.1][pos.0] = b'.';
                pos.0 = (pos.0 as isize + dir.0) as usize;
                pos.1 = (pos.1 as isize + dir.1) as usize;
            }
        }
        Ok(map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| if *c == b'O' { Some(y * 100 + x) } else { None })
                    .sum::<usize>()
            })
            .sum())
    }

    // assert_eq!(2028, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn print(map: &Vec<Vec<u8>>) {
        return;
        let mut lock = stdout().lock();
        for row in map {
            for c in row {
                write!(lock, "{}", *c as char).unwrap();
            }
            writeln!(lock).unwrap();
        }
        lock.flush().unwrap();
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let mut parts = input.split("\n\n");
        let mut map: Vec<Vec<u8>> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'#' => [b'#', b'#'],
                        b'O' => [b'[', b']'],
                        b'@' => [b'@', b'.'],
                        b'.' => [b'.', b'.'],
                        _ => panic!(),
                    })
                    .flatten()
                    .collect()
            })
            .collect();
        let cmds: Vec<u8> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.bytes())
            .flatten()
            .collect();
        let mut pos = map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                if let Some(x) = line.iter().position(|c| *c == b'@') {
                    Some((x, y))
                } else {
                    None
                }
            })
            .unwrap();
        let mut lock = stdout().lock();
        for (i, cmd) in cmds.iter().enumerate() {
            let dir = match cmd {
                b'<' => (-1, 0),
                b'^' => (0, -1),
                b'>' => (1, 0),
                b'v' => (0, 1),
                _ => panic!(),
            };

            fn can_move(
                x: usize,
                y: usize,
                dir: (isize, isize),
                map: &Vec<Vec<u8>>,
                is_other_half: bool,
            ) -> Option<Box<dyn FnMut(&mut Vec<Vec<u8>>, bool)>> {
                let c = map[y][x];
                if c == b'.' {
                    return Some(Box::new(move |_, _| {}));
                }
                if c == b'#' {
                    return None;
                }
                let end_x = (x as isize + dir.0) as usize;
                let end_y = (y as isize + dir.1) as usize;
                let is_vertical = dir.0 == 0;
                let is_box = c == b'[' || c == b']';
                if !is_vertical || !is_box || is_other_half {
                    return if let Some(mut move_next) = can_move(end_x, end_y, dir, map, false) {
                        Some(Box::new(move |map, reapply| {
                            print(map);
                            if !reapply{
                                map[y][x] = b'.';
                            }
                            print(map);
                            move_next(map, reapply);
                            print(map);
                            map[end_y][end_x] = c;
                            print(map);
                        }))
                    } else {
                        None
                    };
                }

                let other_half_dx = match c {
                    b'[' => 1,
                    b']' => -1,
                    _ => panic!(),
                };
                let other_half_x = (x as isize + other_half_dx) as usize;
                if let Some(mut move_next) = can_move(end_x, end_y, dir, map, false) {
                    if let Some(mut move_other_half) = can_move(other_half_x, y, dir, map, true) {
                        return Some(Box::new(move |map, reapply| {
                            print(map);
                            if !reapply{
                                map[y][x] = b'.';
                            }
                            print(map);
                            move_next(map, reapply);
                            move_other_half(map, reapply);
                            map[end_y][end_x] = c;
                            print(map);
                        }));
                    }
                }
                None
            }

            if let Some(mut move_robot) = can_move(pos.0, pos.1, dir, &mut map, false) {
                move_robot(&mut map, false);
                move_robot(&mut map, true);
                pos.0 = (pos.0 as isize + dir.0) as usize;
                pos.1 = (pos.1 as isize + dir.1) as usize;
            }
            
            if i < 1091 {
                continue;
            }
            
            // writeln!(lock)?;
            // writeln!(lock)?;
            // writeln!(lock, "Step {}", i)?;
            // for row in &map {
            //     for c in row {
            //         write!(lock, "{}", *c as char)?;
            //     }
            //     writeln!(lock)?;
            // }
            // lock.flush()?;
            // std::thread::sleep(std::time::Duration::from_millis(800));
        }

        Ok(map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| if *c == b'[' { Some(y * 100 + x) } else { None })
                    .sum::<usize>()
            })
            .sum())
    }

    // part2(BufReader::new(TEST.as_bytes()))?;
    // assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
