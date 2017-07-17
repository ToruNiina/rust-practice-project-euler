extern crate num;
use num::bigint::BigUint;
use num::traits::One;

fn main() {
    let mut n: BigUint = One::one();
    let two = &n + &n;
    for _ in 0..1000 {
        n = n * &two;
    }

    let sum = n.to_str_radix(10).chars().fold(0, |s, d| d.to_digit(10).unwrap() + s);
    println!("the answer is {}", sum);
}
