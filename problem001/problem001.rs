fn main() {
    let mut answer = 0;
    for x in 1..1000 {
        if x % 5 == 0 || x % 3 == 0 {
            answer += x
        }
    }
    println!("answer is: {}", answer);
}
