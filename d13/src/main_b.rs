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

fn tcr((a, m): (i64, i64), (b,n): (i64, i64)) -> (i64, i64) {
    let k = ((a - b) * inv(n, m)).rem_euclid(m);
    (b + n*k, n*m)
}

fn main() {
    let stdin = stdin();
    let input = stdin.lock().lines().nth(1).unwrap().unwrap();

    let an: Vec<(i64, i64)> = input.split(',').enumerate()
        .inspect(|f| eprintln!("{:?}", f))
        .filter_map(|(i, x)| Some(( i as i64, x.parse::<i64>().ok()?)))
        .collect();


    let N = an.iter().map(|(_,n)|n).product();

    let mut x = 0;
    for (a, n) in an {
        let ni = N / n;
        let yi = solve(ni, 1, n);
        eprintln!("{}*x = {} mod {}\n{}", ni, 1, n, yi);
        x = (x + a * ni * yi).rem_euclid(N);
    }
    println!("x: {}", x);
}
