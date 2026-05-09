use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        x: usize,
    }

    println!("{}", a[x - 1]);
}
