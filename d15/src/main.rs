use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let buf = &buf[..buf.len()-1];
    let mut nums: Vec<u64> = buf.split(',').map(|s| s.parse::<u64>().unwrap()).collect();

    let mut last = nums.pop().unwrap();
    let l = nums.len() as u64;
    let mut map: HashMap<u64, u64> = nums.into_iter().enumerate().map(|(i, x)| (x, i as u64)).collect();

    for i in l..29999999 {
        let e = map.entry(last).or_insert(i);
        last = i - *e;
        *e = i;
        if i == 2018 {
            println!("{}", last);
        }
    }
    println!("{}", last);
}
