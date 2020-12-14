use std::io::{BufRead, stdin};
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\]\D+(\d+)").unwrap();
);
const MASK_PREFIX: &'static str = "mask = ";

struct Addr {
    fixed: u64,
    floating: u64,
}

impl Addr {
    pub fn new(addr: u64, or: u64, float: u64) -> Result<Self,()> {
        Ok(Addr{
            fixed: addr | or,
            floating: float,
        })
    }
}

enum Op {
    Mask(u64, u64),
    Mem(u64, u64),
}

impl Op {
    fn create_mask(s: &str) -> Op {
        let mut or = 0;
        let mut float = 0;
        for c in s.chars() {
            match c {
                '1' => {
                    or |= 1;
                }
                '0' => {}
                'X' => float |= 1,
                _ => unreachable!("Invalid mask format"),
            }
            or <<= 1;
            float <<= 1;
        }
        or >>= 1;
        float >>= 1;
        Op::Mask(or, float)
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

struct BitIter {
    num: u64,
    size: usize,
    idx: usize,
}

impl BitIter {
    pub fn new(num: u64, size: usize) -> Self {
        BitIter{
            num, size, idx: 0
        }
    }
}

impl Iterator for BitIter {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.size {
            let m = 1 << self.idx;
            self.idx += 1;
            if self.num & m != 0 {
                return Some(m);
            }
        }
        None
    }
}

fn mem_insert(memmap: &mut HashMap<u64, u64>, addr: Addr, val: u64) {
    let bititer = BitIter::new(addr.floating, 36);
    let v: Vec<u64> = bititer
        .fold(vec![addr.fixed], |mut acc, m| {
            let l = acc.len();
            for k in 0..l {
                acc.push(acc[k] ^ m);
            }
            acc
    });
    for x in v {
        memmap.insert(x, val);
    }
}

fn main() {
    let stdin = stdin();
    let ops = stdin.lock().lines().map(|l| l.unwrap())
    .map(|l| Op::new(&l).unwrap());
    
    let mut memmap: HashMap<u64, u64> = HashMap::new();
    
    let mut or = 0;
    let mut float = 0;

    for op in ops {
        match op {
            Op::Mask(o, f) => {or = o; float = f;},
            Op::Mem(addr, val) => {
                let addr = Addr::new(addr, or, float).unwrap();
                mem_insert(&mut memmap, addr, val);
            }
        }
    }

    let res: u64 = memmap.values().sum();
    println!("{}", res);
}