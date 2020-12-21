use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

use image::{ImageBuffer, Luma};
use image::imageops::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!(
    static ref RE_ID: Regex = Regex::new(r"Tile (\d+):").unwrap();
);

const N: usize = 12;
const M: usize = 10;

enum Side {
    Top,
    Bot,
    Left,
    Right,
}

#[derive(Clone,)]
struct Tile {
    id: u16,
    content: ImageBuffer<Luma<u8>, Vec<u8>>,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tile#")?;
        f.write_fmt(format_args!("{}", self.id))?;
        Ok(())
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

impl std::hash::Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.id);
    }
}

impl Tile {
    fn new(s: &str) -> Self {
        let n: u16 = (&s[5..9]).parse().expect("Cannot parse id");
        
        let mut cnt = 0;

        let vals = s.split_terminator("\n").skip(1)
            .inspect(|_| cnt += 1)
            .flat_map(|l| l.chars().map(|c| if c == '#' {0xffu8} else {0x00u8}))
            .collect();

        let img = ImageBuffer::from_vec(cnt, cnt, vals).expect("Cannot create ImageBuffer");
        
        Self {
            id: n,
            content: img,
        }
    }

    fn symmetries(&self) -> TileSymmetries {
        TileSymmetries{
            state: Symmetry::Iden,
            tile: self.clone(),
        }
    }

    fn matches(&self, other: &Tile, side: Side) -> bool {
        let m = (M - 1) as u32;
        for i in 0..M as u32 {
            match side {
                Side::Top if self.content[(0, i)] != other.content[(m, i)] => return false,
                Side::Bot if self.content[(m, i)] != other.content[(0, i)] => return false,
                Side::Left if self.content[(i, 0)] != other.content[(i, m)] => return false,
                Side::Right if self.content[(i, m)] != other.content[(i, 0)] => return false,
                _ => {}
            }
        }
        true
    }
}

enum Symmetry {
    Iden,
    Rot(u8),
    Flip(u8),
    Transp(u8),
    End,
}

impl Symmetry {
    fn next(&self) -> Self {
        match *self {
            Symmetry::Iden => Self::Rot(0),
            Symmetry::Rot(n) => if n < 2 { Self::Rot(n+1) } else { Self::Flip(0) }
            Symmetry::Flip(n) => if n < 1 { Self::Flip(n+1) } else { Self::Transp(0) }
            Symmetry::Transp(n) => if n < 1 { Self::Transp(n+1) } else { Self::End }
            Symmetry::End => Symmetry::End,
        }
    }
}

struct TileSymmetries {
    state: Symmetry,
    tile: Tile,
}

impl Iterator for TileSymmetries {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.state {
            Symmetry::End => None,
            Symmetry::Iden => {
                Some(self.tile.clone())
            },
            Symmetry::Rot(n) => {
                let img = match n {
                    0 => rotate90(&self.tile.content),
                    1 => rotate180(&self.tile.content),
                    2 => rotate270(&self.tile.content),
                    _ => unreachable!(),
                };
                
                Some(Tile{ id: self.tile.id, content: img,})
            }
            Symmetry::Flip(n) => {
                let img = match n {
                    0 => flip_horizontal(&self.tile.content),
                    1 => flip_vertical(&self.tile.content),
                    _ => unreachable!(),
                };

                Some(Tile{ id: self.tile.id, content: img,})
            }
            Symmetry::Transp(n) => {
                let mut img = rotate90(&self.tile.content);
                match n {
                    0 => flip_horizontal_in_place(&mut img),
                    1 => flip_vertical_in_place(&mut img),
                    _ => unreachable!(),
                };

                Some(Tile{ id: self.tile.id, content: img,})
            }
        };
        self.state = self.state.next();
        ret
    }
}

type PhArray = Vec<Vec<Option<Tile>>>;

fn compatible(arr: &mut PhArray, tile: &Tile, i: usize) -> bool {
    let j = i % N;
    let i = i / N;

    //Left
    if j > 0 {
        if let Some(Some(l)) = arr.get(i).and_then(|r| r.get(j - 1)) {
            if !tile.matches(&l, Side::Left) {return false}
        }
    }
    //Right
    if let Some(Some(l)) = arr.get(i).and_then(|r| r.get(j + 1)) {
        if !tile.matches(&l, Side::Right) {return false}
    }
    //Top
    if i > 0 {
        if let Some(Some(l)) = arr.get(i - 1).and_then(|r| r.get(j)) {
            if !tile.matches(&l, Side::Top) {return false}
        }
    }
    //Bot
    if let Some(Some(l)) = arr.get(i + 1).and_then(|r| r.get(j)) {
        if !tile.matches(&l, Side::Bot) {return false}
    }

    true
}

fn place(arr: &mut PhArray, tiles: &HashMap<u16, Tile>, avail: &mut HashSet<u16>, i: usize) -> bool {
    if i == N*N {return true}
    println!("{:?}", arr.iter().flat_map(|r|r.iter().filter(|&e| e.is_some())).collect::<Vec<_>>());
    let x = i / N;
    let y = i % N;
    assert!(arr[x][y].is_none());

    for id in avail.clone() {
        avail.remove(&id);
        for sym in tiles[&id].symmetries() {
            if compatible(arr, &sym, i) {
                arr[x][y] = Some(sym);
                
                if place(arr, tiles, avail, i + 1) {
                    return true;
                }
            }
        }
        avail.insert(id);
    }
    arr[x][y] = None;
    false
}

fn merge(tiles: Vec<Vec<Tile>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let s = (N * (M-2)) as u32;
    let m = (M - 2) as u32;
    let buf = ImageBuffer::from_fn(s, s, |x, y| {
        let tx = (x/m) as usize;
        let ty = (y/m) as usize;
        let tile = &tiles[tx][ty];
        let xx = (x % m) + 1;
        let yy = (y % m) + 1;

        tile.content[(xx, yy)]
    });

    buf
}

fn sparse_pattern(pat: &str) -> Vec<(u32,u32)> {
    pat.split_terminator('\n')
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter_map(|(i, j, c)| if c == '#' {Some((i as u32, j as u32))} else {None})
        .collect()
}

lazy_static!(
    static ref PATTERN: Vec<(u32, u32)> = sparse_pattern(&"                  # \n#    ##    ##    ###\n #  #  #  #  #  #   \n");
);

const PAT_X: u32 = 20;
const PAT_Y: u32 = 3;

fn mark_if_pat(image: &ImageBuffer<Luma<u8>, Vec<u8>>, mask: &mut Vec<Vec<bool>>, i: u32, j: u32) {
    let mut flag = true;
    for &(x, y) in PATTERN.iter() {
        if image[(i+x, j+y)] == Luma([0]) {
            flag = false;
            break;
        }
    }

    if flag {
        for &(x, y) in PATTERN.iter() {
            mask[(i+x) as usize][(j+y) as usize] = true;
        }
    }
}

fn check_patterns(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<Vec<bool>> {
    let mut mask = vec![vec![false; image.width() as usize] ; image.width() as usize];
    for i in 0..image.width() - PAT_X {
        for j in 0..image.height() - PAT_Y {
            mark_if_pat(image, &mut mask, i, j);
        }
    }
    mask
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let tiles: HashMap<u16, Tile> = buf.split_terminator("\n\n")
        .map(|t| Tile::new(t))
        .map(|t| (t.id, t))
        .collect();

    let mut arr: PhArray = vec![vec![None; N] ; N];
    let mut avail = tiles.keys().map(|k| *k).collect();
    place(&mut arr, &tiles, &mut avail , 0);

    let out: Vec<Vec<Tile>> = arr.into_iter().map(|l| l.into_iter().map(|o| o.unwrap()).collect()).collect();

    println!("{}", out[0][0].id as usize * out[0][N-1].id as usize * out[N-1][ 0].id as usize * out[N-1][N-1].id as usize);

    let merged = merge(out);

    
}
