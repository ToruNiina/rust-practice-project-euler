fn generate_primes_under(x: i64) -> Vec<i64> {
    let upper = (x as f64).sqrt() as i64;
    let mut primes: Vec<i64> = Vec::with_capacity(upper as usize);
    let mut sieve : Vec<i64> = Vec::with_capacity(x as usize);
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

fn is_prime(n: i64, ps: &Vec<i64>) -> bool {
    if n <= 1 {return false;}

    for p in ps.iter() {
        if n <  *p {return false;}
        if n == *p {return true;}
    }
    false
}

fn generate_nums(a: i64, b: i64, ps: &Vec<i64>) -> i64 {
    let mut n: i64 = 0;
    loop {
        let m:i64 = n * n + a * n + b;
        if !is_prime(m, ps) {break;}
        n += 1;
    }
    n
}

fn main() {
    let primes = generate_primes_under(2000000);

    let mut max_len = 0;
    let mut answer  = 0;
    for i in 0..2000 {
        let a: i64 = i - 1000;
        for b in 2..1001 {
            let period = generate_nums(a, b, &primes);
            if max_len < period {
                max_len = period;
                answer = a * b;
            }
        }
    }
    println!("the answer is {}", answer);

}
