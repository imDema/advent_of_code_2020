use std::collections::HashMap;
use std::io::{stdin, Read};

fn exp(a: i64, mut x: i64, n: i64) -> i64 {
    let mut out = 1 as u128;
    let mut p = a as u128;
    while x > 0 {
        if x & 1 > 0 {
            out = (out * p) % n as u128;
        }
        p = (p*p) % n as u128;
        x >>= 1;
    }
    out as i64
}

fn bsgs(alpha: i64, beta: i64, n: i64) -> i64 {
    let m = (n as f32).sqrt().ceil() as i64;

    let mut a = HashMap::new();
    let mut q = 1;
    for j in 0..m {
        a.insert(q, j);
        q = (q * alpha) % n;
    }

    let inv = exp(ALPHA, (-m).rem_euclid(n-1), n);

    let mut y = beta;
    for i in 0..m {
        if let Some(j) = a.get(&y) {
            return i*m + j;
        } else {
            y = (y * inv).rem_euclid(n);
        }
    }
    unreachable!()
}

const N: i64 = 20201227;
const ALPHA: i64 = 7;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<i64> = buf.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap()).collect();

    let (ax, ay) = (input[0], input[1]);


    let x = bsgs(ALPHA, ax, N);
    assert!(exp(ALPHA, x, N) == ax);

    let axy = exp(ay, x, N);

    println!("{}", axy);
}
