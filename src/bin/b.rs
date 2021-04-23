use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i32; n]
    }

    p.sort();

    let slice = &p[0..k];

    let ans = slice.iter().sum::<i32>();

    println!("{}", ans);
}
