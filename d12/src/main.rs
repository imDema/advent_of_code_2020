use std::str::FromStr;
use std::io::{BufRead, stdin};

const ORIENTATION: &[(isize,isize)] = &[(1,0), (0,1), (-1, 0), (0, -1)];

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

    let mut o = 0;
    let mut x = 0;
    let mut y = 0;
    for d in dirs {
        match d {
            Dir::N(v) => y += v,
            Dir::S(v) => y -= v,
            Dir::E(v) => x += v,
            Dir::W(v) => x -= v,
            Dir::L(v) => o = (o + v as usize / 90).rem_euclid(ORIENTATION.len()),
            Dir::R(v) => o = (o as isize - v / 90).rem_euclid(ORIENTATION.len() as isize) as usize,
            Dir::F(v) => {
                x += ORIENTATION[o].0 * v;
                y += ORIENTATION[o].1 * v;
            }
        }
        println!("({},{})", &x, &y);
    }
    println!("{}", x.abs() + y.abs());
}
