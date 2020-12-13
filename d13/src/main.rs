use std::error::Error;
use std::io::{BufRead, stdin};


fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap()?.parse()?;
    let sched = lines.next().unwrap()?;

    let mut min = t;
    let mut wait = t;
    for x in sched.split(',').filter_map(|n| n.parse::<usize>().ok()) {
        let w = (t/x + 1)*x - t;
        if w < wait {
            wait = w;
            min = x;
        }
    }

    println!("{}\n{}", min, min * wait);
    Ok(())
}
