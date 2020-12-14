use std::io::{BufRead, stdin};
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\]\D+(\d+)").unwrap();
);
const MASK_PREFIX: &'static str = "mask = ";


enum Op {
    Mask(u64, u64),
    Mem(u64, u64),
}

impl Op {
    fn create_mask(s: &str) -> Op {
        let mut or = 0;
        let mut and = 0;
        for c in s.chars() {
            match c {
                '1' => {
                    or |= 1;
                    and |= 1;
                }
                '0' => {}
                'X' => and |= 1,
                _ => unreachable!("Invalid mask format"),
            }
            or <<= 1;
            and <<= 1;
        }
        or >>= 1;
        and >>= 1;
        Op::Mask(and, or)
    }

    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.starts_with(MASK_PREFIX) {
            Ok(Self::create_mask(&s[MASK_PREFIX.len()..]))
        } else {
            let caps = RE_MEM.captures(s).ok_or("Invalid format")?;
            let addr: u64 = caps.get(1).unwrap().as_str().parse().map_err(|_| "Invalid addr")?;
            let val: u64 = caps.get(2).unwrap().as_str().parse().map_err(|_| "Invalid val")?;
            Ok(Op::Mem(addr, val))
        }
    }
}

fn main() {
    let stdin = stdin();
    let ops = stdin.lock().lines().map(|l| l.unwrap())
    .map(|l| Op::new(&l).unwrap());
    
    let mut memmap: HashMap<u64, u64> = HashMap::new();
    let mut and = 0;
    let mut or = 0;
    
    for op in ops {
        match op {
            Op::Mask(a, o) => {and = a; or = o;},
            Op::Mem(addr, val) => {
                let v = (val & and) | or;
                memmap.insert(addr, v);
            }
        }
    }

    let res: u64 = memmap.values().sum();

    println!("{}", res);
}