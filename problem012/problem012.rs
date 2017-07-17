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

fn count_pow(mut x: u64, base: u64) -> u64 {
    let mut retval = 0;
    while x % base == 0 {
        x /= base;
        retval += 1;
    }
    retval
}

fn count_factors(x: u64) -> u64 {
    let prime_upper = (x as f64).sqrt() as u64 + 1;
    let primes = generate_primes_under(prime_upper);
    let pows = primes.iter().map(|&p| count_pow(x, p)).collect::<Vec<u64>>();
    pows.iter().fold(1, |p, i| p*(i+1))
}


fn main() {
    println!("number of factors of 12 is {}", count_factors(12));

    for i in 28.. {
        let num = (i+1) * i / 2;
        if count_factors(num) > 500 {println!("the answer is {}", num); break;}
    }
}
