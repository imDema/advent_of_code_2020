use std::str::FromStr;
use std::io::{BufRead, stdin};
use num::Complex;

enum Dir {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

impl FromStr for Dir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: isize = s[1..].parse().map_err(|_| "Invalid value")?;
        let d = match s.chars().nth(0).ok_or("No direction")? {
            'N' => Dir::N(val),
            'S' => Dir::S(val),
            'E' => Dir::E(val),
            'W' => Dir::W(val),
            'L' => Dir::L(val),
            'R' => Dir::R(val),
            'F' => Dir::F(val),
            _ => return Err("Invalid direction"),
        };
        Ok(d)
    }
}

fn main() {
    let stdin = stdin();
    let dirs = stdin.lock().lines().map(|l|l.unwrap())
        .map(|d| d.parse::<Dir>().unwrap());

    let mut z = Complex::new(0,0);
    let mut wz = Complex::new(10, 1);
    for d in dirs {
        match d {
            Dir::N(v) => wz += Complex::new(0,v),
            Dir::S(v) => wz += Complex::new(0,-v),
            Dir::E(v) => wz += Complex::new(v,0),
            Dir::W(v) => wz += Complex::new(-v,0),
            Dir::L(v) => wz *= Complex::i().powu(v as u32 / 90),
            Dir::R(v) => wz *= Complex::i().powu(4 - v as u32 / 90),
            Dir::F(v) => z += wz.scale(v),
        }
        // println!("({})", &z);
    }
    println!("{}", z.re.abs() + z.im.abs());
}
