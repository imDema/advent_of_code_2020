use std::io::BufRead;
use std::cmp::Ordering;

fn main() {
    let stdin = std::io::stdin();
    let mut nums: Vec<u32> = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();
    nums.sort();
    for k in 0..nums.len() {
        let mut i = k+1;
        let mut j = nums.len()-1;
        let z = nums[k];
        while i != j {
            let (x, y) = (nums[i], nums[j]);
            match (x + y + z).cmp(&2020) {
                Ordering::Less => {i += 1}
                Ordering::Greater => {j -= 1}
                Ordering::Equal => {
                    println!("{0} + {1} + {2} = {3}\n{0} * {1} * {2}= {4}",
                        x, y, z, x+y+z, x*y*z);
                    return;
                }
            }
        }
    }
}
