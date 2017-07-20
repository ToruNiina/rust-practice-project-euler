fn is_palindrome(x: String) -> bool {
    let rev = x.chars().rev().collect::<String>();
    x == rev
}

fn is_double_base_palindrome(x:u64) -> bool {
    let base10 = x.to_string();
    let base2  = format!("{:b}", x).to_string();
    is_palindrome(base10) && is_palindrome(base2)
}

fn main() {
    let answer = (1..1000000).filter(|&n| is_double_base_palindrome(n)).sum::<u64>();
    println!("the answer is {}", answer);
}
