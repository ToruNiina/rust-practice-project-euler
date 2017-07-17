fn evenly_divisible_by_1_to_20 (x: usize) -> bool {
    for i in 2..20 {
        if x % i != 0 {return false;}
    }
    true
}

fn main() {
    let upper = 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10 * 11 * 12 * 13 * 14 * 15 * 16 * 17 * 18 * 19 * 20;
    let lower = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    for i in lower..upper {
        if evenly_divisible_by_1_to_20(i) {println!("the answer is {}", i); break;}
    }

}
