use std::io::{BufRead, stdin};
use num::Complex;

fn main() {
    let stdin = stdin();
    let dirs = stdin.lock().lines().map(|l|l.unwrap());

    let mut z = Complex::new(0,0);
    let mut wz = Complex::new(10, 1);
    for d in dirs {
        let v: isize = d[1..].parse().unwrap();
        match d.chars().nth(0).unwrap() {
            'N' => wz += Complex::new(0,v),
            'S' => wz += Complex::new(0,-v),
            'E' => wz += Complex::new(v,0),
            'W' => wz += Complex::new(-v,0),
            'L' => wz *= Complex::i().powu(v as u32 / 90),
            'R' => wz *= Complex::i().powu(4 - v as u32 / 90),
            'F' => z += wz.scale(v),
            _ => unreachable!("Invalid direction"),
        }
    }
    println!("{}", z.re.abs() + z.im.abs());
}
