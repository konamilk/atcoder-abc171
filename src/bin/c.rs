use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    input!{
        mut n : i64
    }
    // let mut v = vec![];

    let mut str = String::new();


    while n > 0 {
        n = n - 1;
        // v.push(n % 26);
        str.push(((n % 26) as u8 + 'a' as u8) as char);
        n = n / 26;
    }

    let reverse = str.chars()
        .into_iter()
        .rev()
        .collect::<String>();


    // println!("{:?}", v);
    println!("{}", reverse);
}
