extern crate num;
use num::bigint::BigUint;
use num::traits::One;
use num::traits::FromPrimitive;

fn count_reciprocal_cycle(x: u64) -> u64 {
    let mut n = x;
    while n % 2 == 0 {n = n / 2;}
    while n % 5 == 0 {n = n / 5;}
    let ten: BigUint = FromPrimitive::from_u32(10).unwrap();

    for i in 1..n {
        let mut tens: BigUint = One::one();
        for _ in 0..i {
            tens = tens * ten.clone();
        }
        if tens % n == One::one() {
            return i;
        }
    }
    0
}

fn main() {
    println!("reciprocal cycle of 1/{} is {}",   3, count_reciprocal_cycle(  3));
    println!("reciprocal cycle of 1/{} is {}",   6, count_reciprocal_cycle(  6));
    println!("reciprocal cycle of 1/{} is {}",   7, count_reciprocal_cycle(  7));
    println!("reciprocal cycle of 1/{} is {}",  17, count_reciprocal_cycle( 17));
    println!("reciprocal cycle of 1/{} is {}", 127, count_reciprocal_cycle(127));

    let cycles = (1..1000).map(|n| count_reciprocal_cycle(n)).collect::<Vec<u64>>();

    let mut answer: u64 = 0;
    let mut maxcyc: u64 = 0;
    for (n, c) in cycles.iter().enumerate() {
        if maxcyc < *c {
            maxcyc = *c;
            answer = n as u64;
        }
    }
    answer += 1;

    println!("the answer is {}, reciprocal cycle is {}", answer, maxcyc);
}
