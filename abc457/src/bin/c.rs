use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        a: [[i64]; n],
        c: [i64; n],
    }

    for i in 0..n {
        let li = a[i].len() as i64;
        let total_elements_in_group = li * c[i];

        if k <= total_elements_in_group {
            let k0 = k - 1;
            let index_in_ai = (k0 % li) as usize;
            println!("{}", a[i][index_in_ai]);
            return;
        } else {
            k -= total_elements_in_group;
        }
    }
}
