fn iterate_collatz_seq(n: u64) -> u64 {
    if n % 2 == 0 {n / 2} else {3 * n + 1}
}

fn count_collatz_seq(mut x: u64) -> u64 {
    let mut count = 1;
    while x != 1 {
        x = iterate_collatz_seq(x);
        count += 1;
    }
    count
}


fn main() {
    println!("{}", count_collatz_seq(13));

    let mut longest = 1;
    let mut longest_chain = 1;

    for i in 2..1000000 {
        let chain = count_collatz_seq(i);
        if longest_chain < chain {
            longest_chain = chain;
            longest = i;
        }
    }
    println!("number that produces longest collatz seq is {}", longest);

}
