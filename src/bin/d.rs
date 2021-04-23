use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use smallvec::alloc::collections::BTreeMap;

fn main() {
    // let source = AutoSource::from("2 1 2 3 1 100 2 100 100 1000");

    input! {
        // from source,
        n: usize,
        a: [i64; n],
        q: usize,
        bc: [(i64, i64); q]
    }

    let mut btm =BTreeMap::<i64, i64>::new();

    for &a_elem in &a{
        let a_value = btm.get(&a_elem).unwrap_or(&0);
        btm.insert(a_elem, a_value + 1);
    }

    let mut ans = a.iter().sum::<i64>();

    for (b_key, c_key) in bc{
        let &b_value = btm.get(&b_key).unwrap_or(&0);
        let &c_value = btm.get(&c_key).unwrap_or(&0);

        if b_value > 0 || c_value > 0 {
            btm.remove(&b_key);
            btm.insert(c_key, b_value + c_value);
        }

        ans += - b_key * b_value;
        ans += b_value * c_key;

        println!("{}", ans);
    }
}
