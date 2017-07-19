fn factorial(x: u64) -> u64 {
    (2..x+1).fold(1, |f, n| f * n)
}

fn calc_digit_factorial(x: u64) -> u64 {
    let x_str = x.to_string();
    x_str.chars().map(|c| factorial(c.to_digit(10).unwrap() as u64)).sum::<u64>()
}


fn is_digit_factorial(x: u64) -> bool {
    x == calc_digit_factorial(x)
}

fn main() {
    println!("the answer is {}", (3..10000000).filter(|&n| is_digit_factorial(n)).sum::<u64>());
}
