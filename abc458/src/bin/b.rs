use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
    }

    for i in 1..=h {
        for j in 1..=w {
            let mut count = 0;
            if i > 1 {
                count += 1;
            }
            if i < h {
                count += 1;
            }
            if j > 1 {
                count += 1;
            }
            if j < w {
                count += 1;
            }

            if j == w {
                println!("{}", count);
            } else {
                print!("{} ", count);
            }
        }
    }
}
