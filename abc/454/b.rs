use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        f: [i32; n],
    }

    let mut seen_values = HashSet::new();
    let mut flag_1 = true;

    for &item in &f {
        if !seen_values.insert(item) {
            flag_1 = false;
        }
    }

    if flag_1 {
        println!("Yes");
    } else {
        println!("No");
    }

    if seen_values.len() >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
