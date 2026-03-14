use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        q: usize,
        query: [(i32, i32); q], // (r, c)
    }

    let mut pre_row = h;
    let mut pre_col = w;

    for (r, c) in query {
        if r == 1 {
            pre_row -= c;
            let answer = c * pre_col;
            println!("{answer}");
        } else {
            pre_col -= c;
            let answer = c * pre_row;
            println!("{answer}");
        }
    }
}
