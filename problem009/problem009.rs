fn is_pythagorean(a: usize, b: usize, c: usize) -> bool {
    c * c == a * a + b * b
}

fn main() {
    for c in (1..997).rev() {
        let upper = 1000 - c;
        for b in (1..upper).rev() {
            let a = upper - b;
            if is_pythagorean(a, b, c) {
                println!("the answer is {}, {}, {} -> {}", a, b, c, a * b * c);
                break;
            }
        }
    }
}
