use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let mut lines = lines.iter();
        let mut computed: HashMap<&str, bool> = HashMap::new();
        lines.by_ref().take_while(|l| !l.is_empty()).for_each(|l| {
            let var = &l[..3];
            let val = &l[5..] == "1";
            computed.insert(var, val);
        });
        let mut zs = Vec::new();
        let map = lines
            .map(|l| {
                let mut parts = l.split(' ');
                let op1 = parts.next().unwrap();
                let operator = parts.next().unwrap();
                let op2 = parts.next().unwrap();
                parts.next();
                let out = parts.next().unwrap();
                if out.starts_with("z") {
                    zs.push(out);
                }
                (
                    out,
                    (
                        op1,
                        op2,
                        match &operator {
                            &"AND" => |op1: bool, op2: bool| op1 & op2,
                            &"OR" => |op1, op2| op1 | op2,
                            &"XOR" => |op1, op2| op1 ^ op2,
                            _ => panic!(),
                        },
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        zs.sort();
        fn compute<'a>(
            var: &'a str,
            computed: &mut HashMap<&'a str, bool>,
            map: &HashMap<&'a str, (&'a str, &'a str, fn(bool, bool) -> bool)>,
        ) -> bool {
            if let Some(val) = computed.get(var) {
                return *val;
            }
            let (op1, op2, f) = map.get(var).unwrap();
            let op1 = compute(op1, computed, map);
            let op2 = compute(op2, computed, map);
            let val = f(op1, op2);
            computed.insert(var, val);
            val
        }
        Ok(zs
            .iter()
            .rev()
            .map(|var| compute(var, &mut computed, &map))
            .fold(0usize, |prev, curr| prev << 1 | curr as usize))
    }

    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
    enum Op {
        And,
        Or,
        Xor,
    }

    fn part2<R: BufRead>(reader: R) -> Result<String> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let mut lines = lines.iter();
        let mut computed: HashMap<&str, bool> = HashMap::new();
        lines.by_ref().take_while(|l| !l.is_empty()).for_each(|l| {
            let var = &l[..3];
            let val = &l[5..] == "1";
            computed.insert(var, val);
        });
        let mut zs = Vec::new();
        let map = lines
            .sorted_by_key(|l| {
                let mut parts = l.split(' ');
                parts.nth(4).unwrap()
            })
            .map(|l| {
                let mut parts = l.split(' ');
                let op1 = parts.next().unwrap();
                let operator = parts.next().unwrap();
                let op2 = parts.next().unwrap();
                let (op1, op2) = (op1.min(op2), op1.max(op2));
                parts.next();
                let out = parts.next().unwrap();
                if out.starts_with("z") {
                    zs.push(out);
                }
                (
                    (
                        op1.to_string(),
                        op2.to_string(),
                        match &operator {
                            &"AND" => Op::And,
                            &"OR" => Op::Or,
                            &"XOR" => Op::Xor,
                            _ => panic!(),
                        },
                    ),
                    out.to_string(),
                )
            })
            .collect::<HashMap<_, _>>();
        zs.sort();
        let mut visited: HashSet<(String, String, Op)> = HashSet::new();
        let mut missing: HashSet<(String, String, Op)> = HashSet::new();
        let mut get = |a: String, b: String, op| {
            let key = if a < b {
                (a.to_string(), b.to_string(), op)
            } else {
                (b.to_string(), a.to_string(), op)
            };
            let res = map.get(&key);
            visited.insert(key.clone());
            if let Some(res) = res {
                Some(res.to_string())
            }
            else {
                missing.insert(key);
                None
            }
        };
        let get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>> =
            &mut Box::new(&mut get);
        fn a(i: usize) -> String {
            format!("x{:02}", i)
        }
        fn b(i: usize) -> String {
            format!("y{:02}", i)
        }
        fn and(
            a: String,
            b: String,
            get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>>,
        ) -> Option<String> {
            get(a, b, Op::And)
        }
        fn or(
            a: String,
            b: String,
            get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>>,
        ) -> Option<String> {
            get(a, b, Op::Or)
        }
        fn xor(
            a: String,
            b: String,
            get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>>,
        ) -> Option<String> {
            get(a, b, Op::Xor)
        }
        // #[rustfmt::skip]
        fn carry(
            i: usize,
            get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>>,
        ) -> Option<String> {
            if i == 0 {
                and(a(i), b(i), get)
            } else {
                or(
                    and(a(i), b(i), get)?,
                    and(xor(a(i), b(i), get)?, carry(i - 1, get)?, get)?,
                    get,
                )
            }
        }
        fn out(
            i: usize,
            get: &mut Box<&mut dyn FnMut(String, String, Op) -> Option<String>>,
        ) -> Option<String> {
            dbg!(i);
            match i {
                0 => xor(a(i), b(i), get),
                45 => carry(i - 1, get),
                _ =>
                xor(
                    xor(
                        a(i),
                        b(i),
                        get
                    )?,
                    carry(i - 1, get)?,
                    get
                ),
            }
        }
        (0..=45).for_each(|i| {
            dbg!(out(i, get));
        });
        for ((op1, op2, op), out) in map.iter().sorted_by_key(|(_, k)| *k) {
            // println!("{} -- {} --> {}", op1, operator, out);
            // println!("{} -- {} --> {}", op2, operator, out);
            let ops = match op {
                Op::And => "AND",
                Op::Or => "OR",
                Op::Xor => "XOR",
            };
            if visited.contains(&(op1.to_string(), op2.to_string(), *op)) {
                println!("{op1} == {ops} ==> {out}");
                println!("{op2} == {ops} ==> {out}");
            }
            else {
                println!("{op1} == {ops} ==> {out}");
                println!("{op2} == {ops} ==> {out}");
            }

        }
        dbg!(map.len());
        dbg!(visited.len());
        dbg!(missing);
        
        // Solution: exported to mermaid, looked for the error around the first missing connection
        // and swapped the wrong ones in the input file, then started again
        
        // qjb XOR mjm
        // qjb AND mjm
        // swapped qjb gvw
        // 
        // rfj OR z15
        // swap z15 jgc
        // 
        // snh OR z22
        // swap z22 drg
        // 
        // grd AND z35
        // grd XOR z35
        // swap jbp z35
        
        Ok("qjb,gvw,z15,jgc,z22,drg,jbp,z35".split(',').sorted().join(","))
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
