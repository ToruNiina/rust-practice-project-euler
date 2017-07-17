fn is_palindromic(x: usize) -> bool {
    x == x.to_string().chars().rev().collect::<String>().parse::<usize>().unwrap()
}

fn main() {
    let mut answer = 0;

    for i in (100..1000).rev() {
        for j in (100..i+1).rev() {
            let mul = i * j;
            if is_palindromic(mul) {
                if answer < mul {answer = mul;}
            }
        }
    }

    println!("the answer is {}", answer);
}
