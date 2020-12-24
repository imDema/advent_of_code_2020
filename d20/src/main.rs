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

type Tile = ImageBuffer<Luma<u8>, Vec<u8>>;

fn parse_image(s: &str) -> Tile {
    let rows: Vec<&str> = s.split_terminator("\n").collect();
        
    let pix: Vec<u8> = rows.iter()
        .inspect(|&r| assert!(r.len() == rows[0].len()))
        .flat_map(|l| l.chars().map(|c| if c == '#' {0xffu8} else {0x00u8}))
        .collect();
    
    ImageBuffer::from_vec(rows[0].len() as u32, rows.len() as u32, pix).expect("Cannot create ImageBuffer")
}

fn parse_tile(s: &str) -> (u16, Tile) {
    let n: u16 = (&s[5..9]).parse().expect("Cannot parse id");
    
    let img_data = &s[s.find('\n').expect("Invalid format")+1..];

    let img = parse_image(img_data);
    
    (n, img)
}

fn tile_symmetries(tile: &Tile) -> TileSymmetries {
    TileSymmetries{
        i: 0,
        j: 0,
        tile: tile.clone(),
    }
}

fn tile_matches(tile: &Tile, other: &Tile, side: Side) -> bool {
    let m = (M - 1) as u32;
    for i in 0..M as u32 {
        match side {
            Side::Top if tile[(0, i)] != other[(m, i)] => return false,
            Side::Bot if tile[(m, i)] != other[(0, i)] => return false,
            Side::Left if tile[(i, 0)] != other[(i, m)] => return false,
            Side::Right if tile[(i, m)] != other[(i, 0)] => return false,
            _ => {}
        }
    }
    true
}


struct TileSymmetries {
    i: u8,
    j: u8,
    tile: Tile,
}

impl Iterator for TileSymmetries {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= 2 {
            return None;
        }

        let ret = self.tile.clone();
        if self.i < 3 {
            self.tile = rotate90(&ret);
            self.i += 1;
        } else if self.j < 2 {
            self.tile = flip_horizontal(&rotate90(&ret));
            self.i = 0;
            self.j += 1;
        }
        Some(ret)
    }
}

type PhArray = Vec<Vec<Option<(u16, Tile)>>>;

macro_rules! get_cell {
    ($arr:ident, $i:expr, $j:expr) => {
        $arr.get($i).and_then(|r| r.get($j))
    };
}

fn compatible(arr: &mut PhArray, tile: &Tile, i: usize) -> bool {
    let j = i % N;
    let i = i / N;

    //Left
    if j > 0 {
        if let Some(Some(l)) = get_cell!(arr, i, j-1) {
            if !tile_matches(tile, &l.1, Side::Left) {return false}
        }
    }
    //Right
    if let Some(Some(l)) = get_cell!(arr, i, j+1) {
        if !tile_matches(tile, &l.1, Side::Right) {return false}
    }
    //Top
    if i > 0 {
        if let Some(Some(l)) = get_cell!(arr, i-1, j) {
            if !tile_matches(tile, &l.1, Side::Top) {return false}
        }
    }
    //Bot
    if let Some(Some(l)) = get_cell!(arr, i+1, j) {
        if !tile_matches(tile, &l.1, Side::Bot) {return false}
    }

    true
}

fn place(arr: &mut PhArray, tiles: &HashMap<u16, Tile>, avail: &mut HashSet<u16>, i: usize) -> bool {
    if i == N*N {return true}
    // println!("{:?}", arr.iter().flat_map(|r|r.iter().filter(|&e| e.is_some())).collect::<Vec<_>>());
    let x = i / N;
    let y = i % N;
    assert!(arr[x][y].is_none());

    for id in avail.clone() {
        avail.remove(&id);
        for sym in tile_symmetries(&tiles[&id]) {
            if compatible(arr, &sym, i) {
                arr[x][y] = Some((id, sym));
                
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

fn merge(tiles: Vec<Vec<(u16,Tile)>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let s = (N * (M-2)) as u32;
    let m = (M - 2) as u32;
    let buf = ImageBuffer::from_fn(s, s, |x, y| {
        let tx = (x/m) as usize;
        let ty = (y/m) as usize;
        let tile = &tiles[tx][ty].1;
        let xx = (x % m) + 1;
        let yy = (y % m) + 1;

        tile[(xx, yy)]
    });

    buf
}

fn mark_if_pat(image: &ImageBuffer<Luma<u8>, Vec<u8>>, mask: &mut Vec<Vec<bool>>, pat: &Tile, i: u32, j: u32) {
    let mut flag = true;
    let path = pat.enumerate_pixels()
        .filter(|&s| *s.2 == Luma([0xff]))
        .map(|(x, y, _)| (i+x, j+y));

    for (x, y) in path.clone() {
        if image[(x, y)] == Luma([0x00]) {
            flag = false;
            break;
        }
    }

    if flag {
        for (x, y) in path {
            mask[x as usize][y as usize] = true;
        }
    }
}

fn check_patterns(image: &Tile, pat: &Tile) -> Vec<Vec<bool>> {
    let mut mask = vec![vec![false; image.width() as usize] ; image.width() as usize];
    for sym in tile_symmetries(pat) {
        for i in 0..image.width() - sym.width() {
            for j in 0..image.height() - sym.height() {
                mark_if_pat(image, &mut mask, &sym, i, j);
            }
        }
    }
    mask
}

const PAT: &str = &"                  # \n#    ##    ##    ###\n #  #  #  #  #  #   \n";

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let tiles: HashMap<u16, Tile> = buf.split_terminator("\n\n")
        .map(parse_tile)
        .collect();

    let mut arr: PhArray = vec![vec![None; N] ; N];
    let mut avail = tiles.keys().map(|k| *k).collect();
    place(&mut arr, &tiles, &mut avail , 0);

    let out: Vec<Vec<(u16,Tile)>> = arr.into_iter().map(|l| l.into_iter().map(|o| o.unwrap()).collect()).collect();

    println!("{}", out[0][0].0 as usize * out[0][N-1].0 as usize * out[N-1][ 0].0 as usize * out[N-1][N-1].0 as usize);

    let merged = merge(out);

    let pat = parse_image(PAT);

    let match_map = check_patterns(&merged, &pat);

    let tot = merged.pixels().filter(|&&p| p == Luma([0xff])).count();
    let monster = match_map.into_iter().flat_map(|v| v.into_iter()).filter(|&b| b).count();

    println!("{}", tot - monster);
}
