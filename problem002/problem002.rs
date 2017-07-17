fn main () {
    let mut x = 1;
    let mut y = 2;
    let mut answer = 2;
    loop {
        let tmp = x+y;
        x = y;
        y = tmp;
        if tmp % 2 == 0  {answer += tmp;}
        if tmp > 4000000 {break}
    }
    println!("answer is: {}", answer);
}
