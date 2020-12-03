use std::io::BufRead;

const X_MOVE: usize = 3;
const Y_MOVE: usize = 1;

fn tree_encounters(map: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    let (mut x, mut y) = (0,0);
    let mut cnt = 0;
    let width = map[0].len();
    while y < map.len() {
        if map[y][x % width] {
            cnt += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    cnt
}

fn main() {
    let stdin = std::io::stdin();
    let map: Vec<Vec<bool>> = stdin.lock().lines().filter_map(|l| l.ok())
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let cnt = tree_encounters(&map, (X_MOVE, Y_MOVE));
    
    println!("{}", cnt);
}
