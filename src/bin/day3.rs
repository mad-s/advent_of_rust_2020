use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let map = stdin.lock().lines().map(Result::unwrap).collect::<Vec<String>>();
    let w = map[0].len();
    let h =  map.len();
    for &slope in &[(1,1), (1,3), (1,5), (1,7), (2,1)] {
        let mut pos = (0, 0);
        let mut trees = 0;
        while pos.0 < h {
            trees += (map[pos.0][pos.1..].chars().next().unwrap() == '#') as u32;
            pos.0 += slope.0;
            pos.1 = (pos.1 + slope.1) % w;
        }
        println!("slope = {:?}", slope);
        println!("{}", trees);
    }

}