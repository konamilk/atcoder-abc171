use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // println!("{}",'a' as i32);  //97
    // println!("{}",'A' as i32);  //65

    input!{
        alpha: char
    }

    if (alpha as i32) < 'a' as i32 {
        println!("A");
    }
    else {
        println!("a");

    }

}
