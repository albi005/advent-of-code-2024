use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 6284759213657113
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut ra: usize = lines[0][12..].parse()?;
        let mut rb: usize = lines[1][12..].parse()?;
        let mut rc: usize = lines[2][12..].parse()?;
        let program: Vec<u8> = lines[4][9..].split(',').map(|instruction| instruction.parse().unwrap()).collect();
        let mut output: Vec<usize> = Vec::new();
        fn get_combo_arg(arg: u8, ra: usize, rb: usize, rc: usize) -> usize {
            match arg {
                0..=3 => arg as usize,
                4 => ra,
                5 => rb,
                6 => rc,
                _ => panic!(),
            }
        }
        let mut p = 0;
        loop {
            if p >= program.len() {
                break;
            }

            let instruction = program[p];
            let arg = program[p + 1];
            match instruction {
                0 => ra >>= get_combo_arg(arg, ra, rb, rc),
                1 => rb ^= arg as usize,
                2 => rb = get_combo_arg(arg, ra, rb, rc) % 8,
                3 => {
                    if ra == 0 {} else {
                        let arg = arg as usize;
                        if p != arg {
                            p = arg;
                            continue;
                        }
                    }
                }
                4 => rb ^= rc,
                5 => output.push(get_combo_arg(arg, ra, rb, rc) % 8),
                6 => rb = ra / 2usize.pow(get_combo_arg(arg, ra, rb, rc) as u32),
                7 => rc = ra / 2usize.pow(get_combo_arg(arg, ra, rb, rc) as u32),
                _ => panic!()
            }
            p += 2;
        }

        Ok(output.iter().join(","))
    }

    fn part1_from_part2<R: BufRead>(reader: R) -> Result<String> {
        let lines: Vec<_> = reader.lines().flatten().collect();
        let mut ra: usize = lines[0][12..].parse()?;
        let mut output: Vec<usize> = Vec::new();
        loop {
            output.push(forward(ra) & 0b111);
            ra >>= 3;
            if ra == 0 { break; }
        }

        Ok(output.iter().join(","))
    }

    assert_eq!(part1(BufReader::new(TEST.as_bytes()))?, part1_from_part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    fn forward(a: usize) -> usize {
        let b = a & 0b111;
        let b = b ^ 0b111;
        let c = a >> b;
        let b = b ^ c;
        let b = b ^ 0b111;
        b
    }

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Program: 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
        //          0   2   4   6   8   10  12  14
        //
        //  0. 2,4 <-\  B = A & 111
        //  2. 1,7   |  B ^= 111
        //  4. 7,5   |  C = A >> B
        //  6. 0,3   |  A >>= 3
        //  8. 4,4   |  B ^= C
        // 10. 1,7   |  B ^= 111
        // 12. 5,5   |  print(B & 111)
        // 14. 3,0 --/ unless A 0

        // let mut a = 0usize;
        // let mut b;
        // let mut c;
        // loop {
        //     b = a & 0b111;
        //     b ^= 0b111;
        //     c = a >> b;
        //     a >>= 3;
        //     b ^= c;
        //     b ^= 0b111;
        //     println!("{}", b & 0b111);
        //     if a == 0 {
        //         break
        //     }
        // }

        if false {
            let mut a = 0usize;
            loop {
                let b = a & 0b111;
                let b = b ^ 0b111;
                let c = a >> b;
                let b = b ^ c;
                let b = b ^ 0b111;
                a >>= 3;
                println!("{}", b & 0b111);
                if a == 0 {
                    break;
                }
            }
        }

        fn forward3(a: usize) -> usize {
            let x1 = a & 0b111;
            let x2 = x1 ^ 0b111;
            let x3 = a >> x2;
            let x4 = x2 ^ x3;
            let x5 = x4 ^ 0b111;
            x5
        }

        fn forward2(a: usize) -> usize {
            ((a & 0b111)
                ^ (a >> ((a & 0b111) ^ 0b111)))
                ^ 0b111
        }

        for output in [0, 1, 2, 3, 4, 5, 6, 7] {
            println!("fw({0:5b}, {0})={1}, >>{2}", (0usize..).find(|&a| forward2(a) == output).unwrap(), output, ((output & 0b111) ^ 0b111));
        }
        println!("cum");

        fn forward1(a: usize) -> usize {
            let mut b;
            b = a & 0b111;
            b ^= 0b111;
            let c = a >> b;
            b ^= c;
            b ^= 0b111;
            b
        }

        for i in 0..8 {
            dbg!(forward(i), forward1(i));
        }

        // ACTUAL SOLUTION // final final
        let lines: Vec<_> = reader.lines().flatten().collect();
        let program: Vec<u8> = lines[4][9..].split(',').map(|instruction| instruction.parse().unwrap()).collect();
        fn rec(a: usize, instructions: &[u8]) -> Option<usize> {
            if instructions.len() == 0 {
                return Some(a);
            }
            let a = a << 3;
            for i in (0..8) {
                let a = a | i;
                let output = forward(a) & 0b111;
                if output == instructions[instructions.len() - 1] as usize {
                    if let Some(a) = rec(a, &instructions[..instructions.len()-1]) {
                        return Some(a);
                    }
                }
            }
            None
        }

        Ok(
            // program.iter().rev().fold(0, |a, instruction| {
            //     let a = a << 3;
            //     ((0usize..8).map(|n| a | n).find(|&a| (forward(a) & 0b111) == *instruction as usize)).unwrap()
            // })
            rec(0, &program).unwrap()
        )
    }

    // assert_eq!(117440, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
