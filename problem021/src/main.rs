
fn list_divisors(x: u64) -> Vec<u64> {
    let mut retval: Vec<u64> = vec![1];
    let upper = (x as f64).sqrt() as u64;

    for i in 2..upper {
        if x % i == 0 {
            retval.push(i);
            retval.push(x / i);
        }
    }
    retval
}

fn d(x:u64) -> u64 {
    list_divisors(x).iter().sum::<u64>()
}

fn has_amicable(x: u64) -> bool {
    let cand = d(x);
    cand != x && d(cand) == x
}

fn main() {
    for i in 1..10000 {
        if has_amicable(i) {
            println!("{} has amicable {}", i, d(i));
        }
    }
    println!("the answer is {}", (1..10000).filter(|n| has_amicable(*n)).sum::<u64>());
}
