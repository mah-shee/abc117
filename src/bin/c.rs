#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [isize; m]
    }
    if n >= m {
        println!("0");
        return;
    }
    x.sort();
    let mut l = vec![0; m - 1];
    for i in 0..m - 1 {
        l[i] = x[i + 1] - x[i];
    }
    l.sort();
    println!(
        "{}",
        x[m - 1] - x[0] - l.iter().rev().take(n - 1).sum::<isize>()
    );
}
