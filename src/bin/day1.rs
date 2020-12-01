use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();
    let values = stdin.lock().lines().map(|x|x.unwrap().parse().unwrap()).collect::<Vec<u32>>();
    println!("{:?}", values);
    let set = values.iter().cloned().collect::<HashSet<u32>>();
    for x in values {
        if set.contains(&(2020-x)) {
            println!("{}", x*(2020-x));
        }
    }
}