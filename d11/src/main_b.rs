use std::io::{BufRead, stdin};
use std::iter::repeat;

use lazy_static::lazy_static;

enum Chair {
    Floor,
    Occupied(u8),
    Free(u8),
}

impl Chair {
    #[inline]
    pub fn bump(&mut self) {
        match *self {
            Chair::Floor => {}
            Chair::Occupied(x) => *self = Chair::Occupied(x+1),
            Chair::Free(x) =>  *self = Chair::Free(x+1),
        }
    }
}

lazy_static!(
    static ref DIRS: Vec<(isize,isize)> = (-1..2).flat_map(|i| repeat(i).zip(-1..2)).filter(|&(i,j)| !(i==0&&j==0)).collect();
);

#[inline]
fn visit_visible<T: Fn(&mut Chair)>(ferry: &mut Vec<Vec<Chair>>, i: usize, j: usize, strategy: T) {
    let (r, c) = (ferry.len() as isize, ferry[0].len() as isize);
    for &(dx, dy) in DIRS.iter() {
        let mut x = i as isize + dx;
        let mut y = j as isize + dy;
        while x>= 0 && x<r && y>=0 && y < c {
            let c = &mut ferry[x as usize][y as usize];
            match c {
                Chair::Floor => {},
                Chair::Free(_) => {strategy(c); break;},
                Chair::Occupied(_) => {strategy(c); break;},
            }
            x += dx;
            y += dy;
        }
    }
}

fn sim(ferry: &mut Vec<Vec<Chair>>) -> bool {
    let mut changed = false;
    for c in ferry.iter_mut().flat_map(|r| r.iter_mut()) {
        *c = match *c {
            Chair::Occupied(x) => if x >= 5 {changed = true; Chair::Free(0)} else {Chair::Occupied(0)}
            Chair::Free(x) => if x == 0 {changed = true; Chair::Occupied(0)} else {Chair::Free(0)}
            Chair::Floor => Chair::Floor,
        }
    }

    for i in 0..ferry.len() {
        for j in 0..ferry[0].len() {
            if let Chair::Occupied(_) = ferry[i][j] {
                visit_visible(ferry, i, j, Chair::bump);
            } 
        }
    }
    changed
}

fn main() {
    let stdin = stdin();

    let mut ferry: Vec<Vec<Chair>> = stdin.lock().lines().map(|l|l.unwrap())
        .map(|l|
            l.chars().map(|c| match c {
                'L' => Chair::Free(0),
                '.' => Chair::Floor,
                _ => unreachable!("Invalid characters"),
            }).collect()).collect();

    while sim(&mut ferry) {}//print(&ferry)}

    let res = ferry.into_iter().fold(0, |acc, row|
        acc + row.into_iter().fold(0, |acc, x| if let Chair::Occupied(_) = x {acc + 1} else {acc})
    );

    println!("{}", res);
}
