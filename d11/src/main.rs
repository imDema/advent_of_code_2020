use std::io::{BufRead, stdin};
use std::iter::repeat;

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

#[inline]
fn visit_neighbours<T: Fn(&mut Chair)>(ferry: &mut Vec<Vec<Chair>>, i: usize, j: usize, strategy: T) {
    let (i0, j0) = (i.saturating_sub(1), j.saturating_sub(1));
    let neigh = (i0..i+2).flat_map(|i| repeat(i).zip(j0..j+2))
        .filter(|&(a, b)| !(a==i && b==j));
    for (i, j) in neigh {
        ferry.get_mut(i)
            .and_then(|r| r.get_mut(j))
            .map(|c| strategy(c));
    }
}

fn sim(ferry: &mut Vec<Vec<Chair>>) -> bool {
    let mut changed = false;
    for c in ferry.iter_mut().flat_map(|r| r.iter_mut()) {
        *c = match *c {
            Chair::Occupied(x) => if x >= 4 {changed = true; Chair::Free(0)} else {Chair::Occupied(0)}
            Chair::Free(x) => if x == 0 {changed = true; Chair::Occupied(0)} else {Chair::Free(0)}
            Chair::Floor => Chair::Floor,
        }
    }

    for i in 0..ferry.len() {
        for j in 0..ferry[0].len() {
            if let Chair::Occupied(_) = ferry[i][j] {
                visit_neighbours(ferry, i, j, Chair::bump);
            } 
        }
    }
    changed
}

fn print(ferry: &Vec<Vec<Chair>>) {
    for r in ferry {
        println!("{}", r.iter().fold(String::new(), |mut acc, x| {
            let c = match *x {
                Chair::Floor => '.',
                Chair::Free(_) => 'L',
                Chair::Occupied(_) => '#',
            };
            acc.push(c);
            acc
        }))
    }
    println!();
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
