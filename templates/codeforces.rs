// https://codeforces.com/problemset/problem/

use std::io::{self, BufRead};

fn solve(lines: &mut io::Lines<io::StdinLock>) {

}

fn main() {
    let stdin: io::Stdin = io::stdin();
    let mut lines: io::Lines<io::StdinLock> = stdin.lock().lines();

    let t: i32 = 1;
    //let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        solve(&mut lines);
    }
}
