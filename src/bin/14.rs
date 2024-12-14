use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_vector(input: &str) -> (isize, isize) {
        let comma = input.find(',').unwrap();
        let x = input[2..comma].parse().unwrap();
        let y = input[comma + 1..].parse().unwrap();
        (x, y)
    }

    fn part1<R: BufRead>(
        reader: R,
        width: isize,
        height: isize,
        iterations: isize,
    ) -> Result<usize> {
        let mid_x = width / 2;
        let mid_y = height / 2;
        let answer = reader
            .lines()
            .flatten()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let pos = parts.next().unwrap();
                let vel = parts.last().unwrap();
                let (x, y) = parse_vector(pos);
                let (dx, dy) = parse_vector(vel);
                let x = (x + dx * iterations).rem_euclid(width);
                let y = (y + dy * iterations).rem_euclid(height);
                (x, y)
            })
            .filter_map(|(x, y)| {
                if x == mid_x || y == mid_y {
                    None
                } else {
                    Some(if x < mid_x && y < mid_y {
                        1
                    } else if y < mid_y {
                        2
                    } else if x < mid_x {
                        3
                    } else {
                        4
                    })
                }
            })
            .counts()
            .iter()
            .map(|pair| *pair.1)
            .product();
        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7, 100)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103, 100)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    // http://cs.bme.hu/bsz1/jegyzet/bsz1_jegyzet.pdf#page=44
    fn solve_linear_congruence_if_a_and_m_are_relative_primes(mut a: isize, b: isize, mut m: isize) -> isize {
        let _m = m;
        let mut p = 0;
        let mut q = b;
        loop {
            let t = m/a;
            let r = m.rem_euclid(a);
            if r == 0 {
                return q.rem_euclid(_m);
            }
            let c = (p-t*q).rem_euclid(_m);
            m = a;
            a = r;
            p = q;
            q = c;
        }
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let robots: Vec<_> = reader
            .lines()
            .flatten()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let pos = parts.next().unwrap();
                let vel = parts.last().unwrap();
                let pos = parse_vector(pos);
                let vel = parse_vector(vel);
                (pos, vel)
            })
            .collect();
        let k = solve_linear_congruence_if_a_and_m_are_relative_primes(103, 41, 101);
        let n = 103 * k + 99;
        assert_eq!(39, n.rem_euclid(101));
        assert_eq!(99, n.rem_euclid(103));

        assert_eq!(39, 544usize.rem_euclid(101));
        assert_eq!(99, 614usize.rem_euclid(103));
        for i in n.. {
            println!();
            println!("{}", i);
            let mut map: Vec<Vec<u8>> = (0..103).map(|_| vec![b'.'; 101]).collect();
            for ((x, y), (dx, dy)) in robots.iter() {
                let x = (x + dx * i).rem_euclid(101) as usize;
                let y = (y + dy * i).rem_euclid(103) as usize;
                map[y][x] = b'#';
            }
            map.iter()
                .for_each(|line| println!("{}", String::from_utf8_lossy(line)));
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
        }
        Ok(0)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
    
    // 305, 342, 408, 443, 511, 
    // hor: 614, 717, 820
    // 614, 512, 409, 306, 203, 100 // :/ -1 hour of my life, thx copilot
    // 100, 203, 306, 409, 512, 614, 717, 820
    // = 100 + 103n
    // vert: 544, 645, 746
    // 544, 443, 342, 241, 140, 39
    // 39, 140, 241, 342, 443, 544, 645, 746
    // = 39 + 101n
    //
    // n === 99 (mod 103)
    // n === 39 (mod 101)
    // ---
    // n = 103k + 99
    // 103k + 100 === 39 (mod 101)
    // 103k === 39 - 99 === -60 (mod 101)
    // 103k === 41 (mod 101)
    
    
    // http://cs.bme.hu/bsz1/jegyzet/bsz1_jegyzet.pdf#subsection.1.4.1
}
