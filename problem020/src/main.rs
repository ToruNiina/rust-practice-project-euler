extern crate num;
use num::bigint::BigUint;
use num::traits::One;
use num::traits::FromPrimitive;

fn main() {
    let mut factorial: BigUint = One::one();

    for i in 1..101 {
        let next: BigUint = FromPrimitive::from_usize(i).unwrap();
        factorial = factorial * next;
    }

    let answer = factorial.to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("the answer is {}", answer);
}
