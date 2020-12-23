use std::io::{stdin, Read};
use std::collections::HashMap;
use std::iter::once;

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

fn sym(maplist: &mut HashMap<u32,u32>, cur: &mut u32, max: u32) {
    let mut z = Vec::new();

    z.push(*maplist.get(cur).unwrap());
    for _ in 0..2 {
        z.push(*maplist.get(z.last().unwrap()).unwrap());
    }

    let mut tar = wrapping_decr(*cur, max);
    while tar == z[0] || tar == z[1] || tar == z[2]  {
        tar = wrapping_decr(tar, max);
    }
    
    let z_index = *maplist.get(cur).unwrap();
    maplist.insert(*cur, *maplist.get(&z[2]).unwrap());
    maplist.insert(z[2], *maplist.get(&tar).unwrap());
    maplist.insert(tar, z_index);
    *cur = *maplist.get(cur).unwrap()
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let input: Vec<u32> = buf.trim().chars()
        .map(|c| c.to_digit(10).expect("Invalid input format") as u32)
        .chain(M1+1..M2+1)
        .collect();

    let mut maplist: HashMap<u32,u32> = input.windows(2)
        .map(|w| (w[0], w[1]))
        .chain(once((M2, input[0])))
        .collect();

    let mut cur = input[0];
    for _ in 0..10000000 {
        sym(&mut maplist, &mut cur, M2);
    }
    let a = *maplist.get(&1).unwrap();
    let b = *maplist.get(&a).unwrap();
    println!("{}", a as usize * b as usize);
}
