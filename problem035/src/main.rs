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

fn is_prime(x: u64, ps: &Vec<u64>) -> bool {
    for p in ps {
        if x <  *p {return false}
        if x == *p {return true}
    }
    false
}

fn is_circular_prime(x: u64, ps: &Vec<u64>) -> bool {
    let mut x_str = x.to_string();
    for _ in 0..x_str.len() {
        let first = x_str.remove(0);
        x_str.push(first);
        if !is_prime(x_str.parse::<u64>().unwrap(), ps) {return false}
    }
    true
}

fn main() {
    let primes = generate_primes_under(1000000);
    println!("the answer is {}", primes.iter().filter(|&&n| is_circular_prime(n, &primes)).map(|&n| n).collect::<Vec<u64>>().len());
}
