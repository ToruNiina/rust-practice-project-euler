fn list_divisors(x: u64) -> Vec<u64> {
    let mut retval: Vec<u64> = vec![1];
    let upper = (x as f64).sqrt() as u64;

    for i in 2..upper+1 {
        if x % i == 0 {
            retval.push(i);
            if x / i != i {
                retval.push(x / i);
            }
        }
    }
    retval
}

fn sum_divisors(x: u64) -> u64 {
    list_divisors(x).iter().sum::<u64>()
}

fn is_abundant(x: u64) -> bool {
    sum_divisors(x) > x
}

fn generate_abundant_under(x: u64) -> Vec<u64> {
    (1..x).filter(|n| is_abundant(*n)).collect::<Vec<u64>>()
}

fn has_value(x: u64, v: &Vec<u64>) -> bool {
    for e in v.iter() {
        if *e == x {return true;}
    }
    false
}

fn can_be_written_as_sum_of_abundants(x: u64, abds: &Vec<u64>) -> bool {
    for c in abds {
        if x <= *c {break;}
        if has_value(x-c, abds) {
            return true;
        }
    }
    false
}

fn main() {
    let abundants = generate_abundant_under(28124);

    let answer = (1..28124)
        .filter(|n| !can_be_written_as_sum_of_abundants(*n, &abundants))
        .sum::<u64>();

    println!("the answer is {}", answer);
}
