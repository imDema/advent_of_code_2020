use std::io::{BufRead, stdin};

fn main() {
    let stdin = stdin();
    let mut adapters: Vec<usize> = stdin.lock().lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut diffs = [0;4];
    diffs[*adapters.first().unwrap()] += 1;
    for w in adapters.windows(2) {
        let diff = w[1]-w[0];
        if diff < diffs.len() {
            diffs[diff] += 1;
        } else {
            panic!("Cannot complete the chain, jolt difference is too high!");
        }
    }
    println!("{:?}\n{}", diffs, diffs[1] * diffs[3]);
}
