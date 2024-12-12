use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map: Vec<Vec<u8>> = reader
            .lines()
            .flatten()
            .map(|line| line.bytes().collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        let mut visited: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        let mut price = 0;

        for y in 0..height {
            for x in 0..width {
                if visited[y][x] {
                    continue;
                }

                let mut region_area = 0;
                let mut region_perimeter = 0;

                fn visit(
                    region_plant: u8,
                    x: isize,
                    y: isize,
                    region_area: &mut usize,
                    region_perimeter: &mut usize,
                    map: &Vec<Vec<u8>>,
                    visited: &mut Vec<Vec<bool>>,
                    width: usize,
                    height: usize,
                ) {
                    if x < 0 || y < 0 {
                        *region_perimeter += 1;
                        return;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    if x >= width || y >= height {
                        *region_perimeter += 1;
                        return;
                    }
                    let plant = map[y][x];
                    if plant != region_plant {
                        *region_perimeter += 1;
                        return;
                    }
                    if visited[y][x] {
                        return;
                    }
                    visited[y][x] = true;
                    *region_area += 1;

                    let x = x as isize;
                    let y = y as isize;
                    let dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
                    for (dx, dy) in dirs {
                        visit(
                            region_plant,
                            x + dx,
                            y + dy,
                            region_area,
                            region_perimeter,
                            map,
                            visited,
                            width,
                            height,
                        );
                    }
                }

                visit(
                    map[y][x],
                    x as isize,
                    y as isize,
                    &mut region_area,
                    &mut region_perimeter,
                    &map,
                    &mut visited,
                    width,
                    height,
                );

                price += region_perimeter * region_area;
            }
        }

        Ok(price)
    }

    // assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map: Vec<Vec<u8>> = reader
            .lines()
            .flatten()
            .map(|line| line.bytes().collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        let mut visited: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        let mut price = 0;

        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        enum BorderSide {
            Up,
            Left,
            Down,
            Right,
        }

        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        struct Border {
            x: isize,
            y: isize,
            side: BorderSide,
        }

        for y in 0..height {
            for x in 0..width {
                if visited[y][x] {
                    continue;
                }

                let mut region_area = 0;
                let mut region_perimeter = 0;
                let mut counted_borders: HashSet<Border> = HashSet::new();

                fn check_perimeter(
                    border: &Border,
                    counted_borders: &mut HashSet<Border>,
                    region_perimeter: &mut usize,
                ) {
                    let dirs: [(isize, isize); 2] = match border.side {
                        BorderSide::Up | BorderSide::Down => [(1, 0), (-1, 0)],
                        BorderSide::Left | BorderSide::Right => [(0, 1), (0, -1)],
                    };
                    let neighbor_1 = Border {
                        x: border.x + dirs[0].0,
                        y: border.y + dirs[0].1,
                        side: border.side,
                    };
                    let neighbor_2 = Border {
                        x: border.x + dirs[1].0,
                        y: border.y + dirs[1].1,
                        side: border.side,
                    };
                    let mut counted_neighbors = 0;
                    if counted_borders.contains(&neighbor_1) {
                        counted_neighbors += 1;
                    }
                    if counted_borders.contains(&neighbor_2) {
                        counted_neighbors += 1;
                    }
                    match counted_neighbors {
                        0 => {
                            *region_perimeter += 1;
                        }
                        1 => {}
                        2 => {
                            *region_perimeter -= 1;
                        }
                        _ => {
                            panic!()
                        }
                    }
                    counted_borders.insert(*border);
                }

                fn visit(
                    region_plant: u8,
                    x: isize,
                    y: isize,
                    border: Border,
                    region_area: &mut usize,
                    region_perimeter: &mut usize,
                    map: &Vec<Vec<u8>>,
                    visited: &mut Vec<Vec<bool>>,
                    width: usize,
                    height: usize,
                    counted_borders: &mut HashSet<Border>,
                ) {
                    if x < 0 || y < 0 {
                        check_perimeter(&border, counted_borders, region_perimeter);
                        return;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    if x >= width || y >= height {
                        check_perimeter(&border, counted_borders, region_perimeter);
                        return;
                    }
                    let plant = map[y][x];
                    if plant != region_plant {
                        check_perimeter(&border, counted_borders, region_perimeter);
                        return;
                    }
                    if visited[y][x] {
                        return;
                    }
                    visited[y][x] = true;
                    *region_area += 1;

                    let x = x as isize;
                    let y = y as isize;
                    let dirs = [
                        (1, 0, BorderSide::Right),
                        (-1, 0, BorderSide::Left),
                        (0, 1, BorderSide::Down),
                        (0, -1, BorderSide::Up),
                    ];
                    for (dx, dy, side) in dirs {
                        let border = Border {
                            x,
                            y,
                            side,
                        };
                        visit(
                            region_plant,
                            x + dx,
                            y + dy,
                            border,
                            region_area,
                            region_perimeter,
                            map,
                            visited,
                            width,
                            height,
                            counted_borders,
                        );
                    }
                }

                visit(
                    map[y][x],
                    x as isize,
                    y as isize,
                    Border {
                        x: isize::MAX,
                        y: isize::MAX,
                        side: BorderSide::Left,
                    }, // doesn't matter, won't be read
                    &mut region_area,
                    &mut region_perimeter,
                    &map,
                    &mut visited,
                    width,
                    height,
                    &mut counted_borders,
                );

                price += region_perimeter * region_area;
            }
        }

        Ok(price)
    }

    assert_eq!(368, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
