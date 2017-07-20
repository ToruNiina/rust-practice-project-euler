fn generate_primes_under(x: u64) -> Vec<u64> {
    let upper = (x as f64).sqrt() as u64;
    let mut primes: Vec<u64> = Vec::with_capacity(upper as usize);
    let mut sieve : Vec<u64> = Vec::with_capacity(x as usize);
    for i in 2..x {
        sieve.push(i);
    }
    loop {
        if sieve[0] > upper {break;}

        let head = sieve[0];
        primes.push(head);
        sieve.retain(|&x| x % head != 0);
    }
    primes.append(&mut sieve);
    primes
}

fn is_prime(x: u64, primes: &Vec<u64>) -> bool {
    for p in primes {
        if x <  *p {return false;}
        if x == *p {return true;}
    }
    false
}

fn is_truncatable_prime(x: u64, primes: &Vec<u64>) -> bool {
    let x_str = x.to_string();
    if !is_prime(x, primes) {return false;}

    for i in 1..x_str.len() {
        let (tmp1, tmp2) = x_str.split_at(i);
        if !is_prime(tmp1.parse::<u64>().unwrap(), primes) ||
           !is_prime(tmp2.parse::<u64>().unwrap(), primes) {return false;}
    }
    true
}

fn main() {
    let upper = 1000000;
    let primes = generate_primes_under(upper);
    let answer: u64 = primes.iter()
        .filter(|&&p| p > 10 && is_truncatable_prime(p, &primes))
        .sum();

    println!("the answer is {}", answer);
}
