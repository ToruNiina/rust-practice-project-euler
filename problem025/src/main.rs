extern crate num;
use num::bigint::BigUint;
use num::traits::One;

fn main() {
    let mut x: BigUint = One::one(); // F1
    let mut y: BigUint = One::one(); // F2
    let mut i = 2;

    loop {
        let tmp = y.clone();
        y = x + y;
        x = tmp;
        i += 1;

        if i < 10 {
            println!("{}-th F is {}", i, y);
        }


        if y.to_str_radix(10).len() >= 1000 {
            println!("the answer is {}", i);
            break;
        }
    }
}
