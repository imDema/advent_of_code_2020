use std::io::stdin;

fn run(seed: &Vec<usize>, n: usize) -> usize {
    let mut last_seen: Vec<i32> = vec![-1; n];
    for (i, &x) in seed[..seed.len()-1].iter().enumerate() {
        last_seen[x] = i as i32;
    }
    let mut last = *seed.last().unwrap();
    for i in seed.len()-1..n-1 {
        let ls = last_seen[last];
        last_seen[last] = i as i32;
        last = if ls >= 0{
            i - ls as usize
        } else {
            0
        };
    }
    last
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let buf = &buf[..buf.len()-1];
    
    let nums: Vec<usize> = buf.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    let last = run(&nums, 30000000);

    println!("{}", last);
}
