use std::io::{BufRead, stdin};

fn inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
   
    while mn.1 != 0 {
      xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
      mn = (mn.1, mn.0 % mn.1);
    }
   
    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0
}

fn solve(n: i64, a: i64, module: i64) -> i64 {
    let ninv = inv(n, module);
    (a*ninv).rem_euclid(module)
}

fn main() {
    let stdin = stdin();
    let input = stdin.lock().lines().nth(1).unwrap().unwrap();

    let an: Vec<(i64, i64)> = input.split(',').enumerate()
        .filter_map(|(i, x)| Some(( i as i64, x.parse::<i64>().ok()?)))
        .map(|(a,n)|((-a).rem_euclid(n), n))
        .collect();

    let big_n = an.iter().map(|(_,n)|n).product();

    let mut x = 0;
    for (a, n) in an {
        let ni = big_n / n;
        let yi = solve(ni, 1, n);
        x = (x + a * ni * yi).rem_euclid(big_n);
    }
    println!("x: {}", x);
}
