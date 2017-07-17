
fn generate_primes_under(x: usize) -> Vec<usize> {
    let upper = (x as f64).sqrt() as usize;
    let mut primes: Vec<usize> = Vec::with_capacity(upper);
    let mut sieve : Vec<usize> = Vec::with_capacity(x);
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

fn main() {
    let mut upper = 1000000;
    loop {
        let primes = generate_primes_under(upper);
        if primes.len() >= 10000 {
            println!("the answer is {}", primes[10000]);
            break;
        }
        upper *= 10;
    }
}
