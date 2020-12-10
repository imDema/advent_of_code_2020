use std::io::{BufRead, stdin};

fn main() {
    let stdin = stdin();
    let mut adapters: Vec<usize> = stdin.lock().lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut paths: Vec<usize> = (0..adapters.len()).map(|_| 0).collect();
    paths[0] = 1;
    for i in 0..paths.len() {
        for j in i+1..(i+4).min(adapters.len()) {
            if adapters[j] - adapters[i] <= 3 {
                paths[j] = paths[j].checked_add(paths[i]).unwrap();
            }
        }
    }
    // println!("{:?}...{:?}", &paths[..10], &paths[paths.len()-11..]);
    println!("{}", paths.last().unwrap());
}
