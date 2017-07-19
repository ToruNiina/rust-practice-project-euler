fn main() {
    let mut n = 0;
    for a in 0..2   { // 200
    for b in 0..3   { // 100
    for c in 0..5   { //  50
    for d in 0..11  { //  20
    for e in 0..21  { //  10
    for f in 0..41  { //   5
    for g in 0..101 { //   2
        if a * 200 + b * 100 + c * 50 + d * 20 + e * 10 + f * 5 + g * 2 <= 200 {
            n += 1;
        }
    }
    }
    }
    }
    }
    }
    }
    println!("the answer is {} ", n);
}
