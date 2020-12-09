use std::io::BufRead;

fn find_seq(values: &[u64], x: u64) -> Option<&[u64]> {
    let (mut i, mut j) = (0, 1);
    let mut acc = values.get(i)? + values.get(j)?;
    loop {
        match acc.cmp(&x) {
            std::cmp::Ordering::Equal => return Some(&values[i..j]),
            std::cmp::Ordering::Less => {
                j += 1;
                acc += values.get(j)?;
            }
            std::cmp::Ordering::Greater => {
                if i < j - 1 {
                    i += 1;
                    acc -= values[i-1];
                } else {
                    i += 1;
                    j += 1;
                    acc += values.get(j)? - values[i-1];
                }
            }
        }
    }
}

fn is_sum(values: &[u64], x: u64) -> bool {
    for i in 0..values.len() {
        for j in i+1..values.len() {
            if values[i] + values[j] == x {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let stdin = std::io::stdin();
    let values: Vec<u64> = stdin.lock().lines().map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap()).collect();

    let mut k = None;
    let mut n = None;
    for (i, v) in values.iter().enumerate().skip(25) {
        if !is_sum(&values[i-25..i], *v) {
            println!("{}", v);
            k = Some(*v);
            n = Some(i);
            break;
        }
    }

    let seq = find_seq(&values[..n.unwrap()], k.unwrap()).unwrap();
    let (min, max) = seq.iter().fold((seq[0], seq[0]), |mut mima, &x| {
        if x < mima.0 { mima.0 = x }
        if x > mima.1 { mima.1 = x }
        mima
    });
    println!("{}", min + max);
}