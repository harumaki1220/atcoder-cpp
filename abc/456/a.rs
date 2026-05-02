use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    if x > 18 || x < 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
