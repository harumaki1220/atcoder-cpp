use proconio::{input, marker::Chars};
use std::cmp::min;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: usize = 0;

    for i in 0..s.len() {
        if s[i] == 'C' {
            let left = i;
            let right = s.len() - 1 - i;
            ans += min(left, right) + 1;
        }
    }

    println!("{}", ans);
}
