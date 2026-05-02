use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let m: i64 = 998244353;
    let mut ans: i64 = 0;
    let mut length: i64 = 0;

    for i in 0..n {
        if i == 0 || s[i] != s[i - 1] {
            length += 1;
        } else {
            ans += length * (length + 1) / 2;
            length = 1;
        }
    }

    ans = (ans + length * (length + 1) / 2) % m;

    println!("{}", ans);
}
