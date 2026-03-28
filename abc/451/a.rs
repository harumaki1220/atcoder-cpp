use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s.len() == 5 || s.len() == 10 {
        println!("Yes");
    } else {
        println!("No");
    }
}
