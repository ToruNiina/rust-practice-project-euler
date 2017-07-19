extern crate num;
use num::bigint::BigUint;
use num::traits::FromPrimitive;

fn main() {

    let mut nums: Vec<BigUint> = Vec::new();
    for i in 2..101 {
        let a: BigUint = FromPrimitive::from_usize(i).unwrap();
        let mut n = a.clone();
        for _ in 2..101 {
            n = n.clone() * a.clone();
            nums.push(n.clone());
        }
    }
    nums.sort();
    nums.dedup();
    println!("the answer is {}", nums.len());
}
