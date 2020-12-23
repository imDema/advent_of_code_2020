use std::io::{stdin, Read};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

type Input = Vec<u32>;

const M1: u32 = 9;
const M2: u32 = 1000000;

#[inline]
fn wrapping_decr(x: u32, max: u32) -> u32 {
    if x > 1 {
        x - 1
    } else {
        max
    }
}

fn sym(cups: &mut Input, max: u32) {
    let current = cups[0];
    let z = (cups[1], cups[2], cups[3]);

    let mut target = wrapping_decr(cups[0], max);

    while target == z.0 || target == z.1 || target == z.2 {
        target = wrapping_decr(cups[0], max);
    }

    let dest = cups.iter().enumerate().find(|&x| *x.1 == target).map(|x|x.0).unwrap();

    cups[1..dest-3].copy_from_slice(&cups[4..dest]);
    for i in 4.. {
        cups[i-4] = cups[i];
        if cups[i] == target {
            dest = Some(i);
            break;
        }
    };
    let dest = dest.unwrap();
    cups[dest-3] = z.0;
    cups[dest-2] = z.1;
    cups[dest-1] = z.2;

    for i in dest+1..cups.len() {
        cups[i-1] = cups[i];
    }
    let last = cups.len()-1;
    cups[last] = current;
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut cups: Input = buf.trim().chars().map(|c| c.to_digit(10).expect("Invalid input format") as u32).collect();

    // Part 1
    for _ in 0..100 {
        sym(&mut cups, M1);
    }
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
    let out = cups.iter().skip(1).fold(String::new(), |mut acc, x| {
        acc.push_str(&format!("{}", x));
        acc
    });
    println!("{}", out);

    // Part 2
    let mut cups: Input = buf.trim()
        .chars().map(|c| c.to_digit(10).expect("Invalid input format") as u32)
        .chain(M1+1..M2+1)
        .collect();

    let mut last = 0;
    let mut last_t = Instant::now();

    for i in 0..10000000 {
        if last_t.elapsed().as_secs_f32() > 1.0 {
            eprintln!("{}", i-last);
            last = i;
            last_t = Instant::now();
        }

        sym(&mut cups, M1);
    }
    let out: usize = cups.iter().skip_while(|&&x| x != 1)
        .skip(1).take(2).map(|&x| x as usize).product();

    println!("{}", out);
}
