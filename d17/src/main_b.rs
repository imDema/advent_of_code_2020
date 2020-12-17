use std::io::{stdin, BufRead};
use std::collections::HashMap;

type Coord = (isize, isize, isize, isize);
type Cube = HashMap<Coord, Cubelet>;

#[derive(Copy, Clone)]
struct Cubelet {
    active: bool,
    neigh: u8,
}

impl Cubelet {
    pub fn bump(&mut self) {
        self.neigh += 1;
    }
    pub fn bump_down(&mut self) {
        self.neigh = self.neigh.checked_sub(1).unwrap();
    }
}

impl Default for Cubelet {
    fn default() -> Self {
        Self{active: false, neigh: 0}
    }
}

fn bump_neigh(cube: &mut Cube, coord: Coord) {
    let xi = coord.0-1..coord.0+2;
    let yi = coord.1-1..coord.1+2;
    let zi = coord.2-1..coord.2+2;
    let wi = coord.3-1..coord.3+2;
    for x in xi {
        for y in yi.clone() {
            for z in zi.clone() {
                for w in wi.clone() {
                    let cblt = cube.entry((x,y,z,w)).or_default();
                    cblt.bump();
                }
            }
        }
    }
    cube.get_mut(&coord).unwrap().bump_down();
}

fn activate_cubelet(cube: &mut Cube, coord: Coord) {
    let cblt = cube.entry(coord).or_default();
    cblt.active = true;
    bump_neigh(cube, coord);
}

fn sim(cube: &mut Cube) {
    let mut cube2: Cube = HashMap::new();
    for (&coord, cblt) in cube.iter() {
        if cblt.active {
            if cblt.neigh == 2 || cblt.neigh == 3 {
                activate_cubelet(&mut cube2, coord);
            }
        } else {
            if cblt.neigh == 3 {
                activate_cubelet(&mut cube2, coord);
            }
        }
    }
    *cube = cube2;
}

fn sim_n(cube: &mut Cube, n: usize) {
    for _ in 0..n {
        sim(cube);
    }
}

fn main() {
    let stdin = stdin();
    let mut cube: Cube = HashMap::new();
    for (i, l) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                let coord = (i as isize, j as isize, 0, 0);
                activate_cubelet(&mut cube, coord);
            }
        }
    }
    sim_n(&mut cube, 6);

    let count = cube.into_iter().filter(|(_, cb)| cb.active).count();
    println!("{}", count);
}
