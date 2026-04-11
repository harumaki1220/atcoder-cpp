use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut found_non_o = false;
    for &c in &s {
        if c != 'o' {
            found_non_o = true;
        }

        if found_non_o {
            print!("{}", c);
        }
    }
    println!("");
}
