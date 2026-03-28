use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n], // (A, B)
    }

    let mut diff = vec![0; m];

    for (a, b) in ab {
        diff[a - 1] -= 1;

        diff[b - 1] += 1;
    }

    for result in diff {
        println!("{result}");
    }
}
