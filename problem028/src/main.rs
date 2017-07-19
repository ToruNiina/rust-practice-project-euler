fn pow2(x:u64) -> u64 {
    x * x
}

fn main() {
    let answer:u64 = (1u64..501u64)
//     let answer:u64 = (1u64..3u64)
        .map(|k| (pow2(2*k) + 1u64 + pow2(2*k+1)) - 2*k)
        .sum();
    println!("the answer is {}", 2*answer+1u64);
}
