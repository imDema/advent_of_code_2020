use std::io::{stdin, Read};
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

fn sym(maplist: &mut Vec<u32>, cur: &mut u32, max: u32) {
    let mut z = Vec::new();

    z.push(*maplist.get(*cur as usize as usize).unwrap());
    for _ in 0..2 {
        z.push(*maplist.get(*z.last().unwrap() as usize).unwrap());
    }

    let mut tar = wrapping_decr(*cur, max);
    while tar == z[0] || tar == z[1] || tar == z[2]  {
        tar = wrapping_decr(tar, max);
    }
    
    let z_index = *maplist.get(*cur as usize).unwrap();
    maplist[*cur as usize] = *maplist.get(z[2] as usize).unwrap();
    maplist[z[2] as usize] = *maplist.get(tar as usize).unwrap();
    maplist[tar as usize] = z_index;
    *cur = *maplist.get(*cur as usize).unwrap()
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let input: Vec<u32> = buf.trim().chars()
        .map(|c| c.to_digit(10).expect("Invalid input format") as u32)
        .chain(M1+1..M2+1)
        .collect();


    let mut maplist: Vec<u32> = vec![0; M2 as usize + 1]; // Use base 1 indexing because this is a challenge and not a serious project
    
    for w in input.windows(2) {
        maplist[w[0] as usize] = w[1];
    }
    *maplist.last_mut().unwrap() = input[0];

    let mut cur = input[0];
    for _ in 0..10000000 {
        sym(&mut maplist, &mut cur, M2);
    }
    let a = *maplist.get(1 as usize).unwrap();
    let b = *maplist.get(a as usize).unwrap();
    println!("{}", a as usize * b as usize);
}
