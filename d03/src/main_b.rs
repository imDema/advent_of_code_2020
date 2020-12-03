use std::io::BufRead;

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

    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let cnt = slopes.into_iter()
        .fold(1, |acc, s| acc * tree_encounters(&map, s));
    
    println!("{}", cnt);
}
