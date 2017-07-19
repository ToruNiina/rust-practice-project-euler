fn fifthpow(x: u64) -> u64 {
    x * x * x * x * x
}

fn sum_digit_fifthpow(x: u64) -> u64 {
    x.to_string().chars().map(|c| c.to_digit(10).unwrap() as u64).map(|n| fifthpow(n)).sum::<u64>()
}

fn main() {
    let answer: u64 = (1u64..1000000u64).filter(|&n| n == sum_digit_fifthpow(n)).sum();
    println!("the answer is {}", answer - 1);
}
