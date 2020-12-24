use std::collections::HashMap;
use std::io::{stdin, Read};

type Floor = HashMap<Coord, Tile>;

#[derive(Copy, Clone)]
struct Tile {
    active: bool,
    neigh: u8,
}

impl Tile {
    pub fn bump(&mut self) {
        self.neigh += 1;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self{active: false, neigh: 0}
    }
}

const NEIGH: &[Coord] = &[(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];

fn bump_neigh(floor: &mut Floor, coord: Coord) {
    let neighs = NEIGH.iter().map(|c| (c.0 + coord.0, c.1+coord.1));
    for x in neighs {
        let tile = floor.entry(x).or_default();
        tile.bump();
    }
}

fn activate_tile(floor: &mut Floor, coord: Coord) {
    let tile = floor.entry(coord).or_default();
    tile.active = true;
    bump_neigh(floor, coord);
}

fn sim(floor: &mut Floor) {
    let mut floor2: Floor = HashMap::new();
    for (&coord, tile) in floor.iter() {
        if tile.active {
            if tile.neigh == 1 || tile.neigh == 2 {
                activate_tile(&mut floor2, coord);
            }
        } else {
            if tile.neigh == 2 {
                activate_tile(&mut floor2, coord);
            }
        }
    }
    *floor = floor2;
}

fn sim_n(floor: &mut Floor, n: usize) {
    for _ in 0..n {
        sim(floor);
    }
}


type Coord = (i16, i16);

fn parse_sequence(s: &str) -> Coord {
    let mut chars = s.chars();
    let (mut e, mut ne) = (0, 0);
    while let Some(c) = chars.next() {
        match c {
            'e' => e += 1,
            'w' => e -= 1,
            'n' => {
                ne += 1;
                match chars.next() {
                    Some('e') => {}
                    Some('w') => e -= 1,
                    _ => unreachable!()
                }
            }
            's' => {
                ne -= 1;
                match chars.next() {
                    Some('e') => e += 1,
                    Some('w') => {}
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
    (e, ne)
}

fn parse_input(s: &str) -> Vec<Coord> {
    s.lines()
    .map(parse_sequence)
    .collect()
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    // Part 1
    let mut flip_coords: Vec<Coord> = parse_input(&buf);
    flip_coords.sort();

    let mut room: HashMap<Coord, bool> = HashMap::new();
    for &c in flip_coords.iter() {
        let e = room.entry(c).or_insert(false);
        *e = !*e;
    }

    let res = room.values().filter(|&&v|v).count();
    println!("{}", res);

    // Part 2
    let mut floor = Floor::new();
    for (c, _) in room.into_iter().filter(|&(_, v)| v) {
        activate_tile(&mut floor, c);
    }

    sim_n(&mut floor, 100);

    let res = floor.values().filter(|&&v| v.active).count();
    println!("{}", res);
}
