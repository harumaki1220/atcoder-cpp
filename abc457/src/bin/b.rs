use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    for _ in 0..n {
        input! {
            l: usize,
            row: [i32; l],
        }
        a.push(row);
    }

    input! {
        x: usize,
        y: usize,
    }

    println!("{}", a[x - 1][y - 1]);
}
