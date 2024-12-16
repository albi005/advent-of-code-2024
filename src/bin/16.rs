use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();
        let mut lowest_scores: HashMap<(usize, usize, Dir), usize> = HashMap::new();
        let mut queue: VecDeque<(usize, usize, Dir, usize)> = VecDeque::new();
        queue.push_back((start.0, start.1, Dir::Right, 0));
        let mut lowest_score = usize::MAX;
        while queue.len() > 0 {
            let (x, y, dir, score) = queue.pop_front().unwrap();
            let c = map[y][x];
            match c {
                '#' => {}
                'E' => {
                    lowest_score = lowest_score.min(score);
                }
                'S' | '.' => {
                    if let Some(&prev_score) = lowest_scores.get(&(x, y, dir)) {
                        if score >= prev_score {
                            continue;
                        }
                    }
                    lowest_scores.insert((x, y, dir), score);

                    let turns = match dir {
                        Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
                        Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
                    };
                    for &new_dir in turns.iter() {
                        queue.push_back((x, y, new_dir, score + 1000));
                    }

                    let (fw_dx, fw_dy) = match dir {
                        Dir::Up => (0, -1),
                        Dir::Down => (0, 1),
                        Dir::Left => (-1, 0),
                        Dir::Right => (1, 0),
                    };
                    let fw_x = (x as isize + fw_dx) as usize;
                    let fw_y = (y as isize + fw_dy) as usize;
                    queue.push_back((fw_x, fw_y, dir, score + 1));
                }
                _ => panic!(),
            }
        }

        Ok(lowest_score)
    }

    assert_eq!(7036, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();
        let mut lowest_scores: HashMap<(usize, usize, Dir), (usize, HashSet<(usize, usize, Dir)>)> =
            HashMap::new();
        let mut queue: VecDeque<(usize, usize, Dir, usize, (usize, usize, Dir))> = VecDeque::new();
        queue.push_back((start.0, start.1, Dir::Right, 0, (0, 0, Dir::Right)));
        let mut lowest_score = usize::MAX;
        let mut end_prev_states = HashSet::new();
        while queue.len() > 0 {
            let (x, y, dir, score, prev_state) = queue.pop_front().unwrap();
            let c = map[y][x];
            match c {
                '#' => {}
                'E' => {
                    if score < lowest_score {
                        lowest_score = score;
                        end_prev_states = HashSet::new();
                    }
                    if score == lowest_score {
                        end_prev_states.insert(prev_state);
                    }
                }
                'S' | '.' => {
                    if let Some((prev_score, prev_states)) = lowest_scores.get_mut(&(x, y, dir)) {
                        if score > *prev_score {
                            continue;
                        }
                        if score == *prev_score {
                            prev_states.insert(prev_state);
                            continue;
                        }
                    }
                    lowest_scores.insert(
                        (x, y, dir),
                        (score, {
                            let mut set = HashSet::new();
                            set.insert(prev_state);
                            set
                        }),
                    );

                    let turns = match dir {
                        Dir::Up | Dir::Down => [Dir::Left, Dir::Right],
                        Dir::Left | Dir::Right => [Dir::Up, Dir::Down],
                    };
                    for &turned_dir in turns.iter() {
                        queue.push_back((x, y, turned_dir, score + 1000, (x, y, dir)));
                    }

                    let (fw_dx, fw_dy) = match dir {
                        Dir::Up => (0, -1),
                        Dir::Down => (0, 1),
                        Dir::Left => (-1, 0),
                        Dir::Right => (1, 0),
                    };
                    let fw_x = (x as isize + fw_dx) as usize;
                    let fw_y = (y as isize + fw_dy) as usize;
                    queue.push_back((fw_x, fw_y, dir, score + 1, (x, y, dir)));
                }
                _ => panic!(),
            }
        }

        let mut best_spots: HashSet<(usize, usize)> = HashSet::new();
        fn visit(
            x: usize,
            y: usize,
            dir: Dir,
            lowest_scores: &HashMap<(usize, usize, Dir), (usize, HashSet<(usize, usize, Dir)>)>,
            best_spots: &mut HashSet<(usize, usize)>,
        ) {
            if let Some((score, prev_states)) = lowest_scores.get(&(x, y, dir)) {
                best_spots.insert((x, y));
                if *score == 0 {
                    return;
                }
                for &(prev_x, prev_y, prev_dir) in prev_states {
                    visit(prev_x, prev_y, prev_dir, lowest_scores, best_spots);
                }
            }
        }
        for (x, y, dir) in end_prev_states {
            visit(x, y, dir, &lowest_scores, &mut best_spots);
        }

        Ok(best_spots.len() + 1)
    }

    assert_eq!(45, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
