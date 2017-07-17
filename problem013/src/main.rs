extern crate num;
use num::bigint::BigUint;
use num::traits::Zero;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("problem013.dat").unwrap();
    let buf  = BufReader::new(&file);

    let nums = buf.lines()
        .map(|l| BigUint::parse_bytes(l.unwrap().as_bytes(), 10).unwrap())
        .collect::<Vec<BigUint>>();

    let mut sum: BigUint = Zero::zero();
    for v in &nums {
        sum = sum + v;
    }

    println!("the answer is {}", sum);
}
