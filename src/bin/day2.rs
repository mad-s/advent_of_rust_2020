use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", stdin.lock().lines().filter(|line| {
        let line = line.as_ref().unwrap();
        let mut parts = line.split(':');
        let policy = parts.next().unwrap().trim();
        let password = parts.next().unwrap().trim();
        let mut parts = policy.split(' ');
        let counts = parts.next().unwrap().trim();
        let letter = parts.next().unwrap().trim().chars().next().unwrap();
        let mut parts = counts.split('-');
        let min : usize = parts.next().unwrap().parse().unwrap();
        let max : usize = parts.next().unwrap().parse().unwrap();
        let occurrences = password.chars().filter(|&x| x == letter).count();
        //(min..=max).contains(&occurrences)
          (password[min-1..].chars().next().unwrap() == letter)
        ^ (password[max-1..].chars().next().unwrap() == letter)
    }).count());
}