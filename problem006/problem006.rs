
fn sum_of_squares(lower: usize, upper: usize) -> usize {
    let mut retval = 0;
    for i in lower..upper+1 {
        retval += i * i;
    }
    retval
}

fn square_of_sum(lower: usize, upper: usize) -> usize {
    let mut retval = 0;
    for i in lower..upper+1 {
        retval += i;
    }
    retval * retval
}

fn main() {
    let sum_of_squares = sum_of_squares(1, 100);
    let square_of_sum  = square_of_sum(1, 100);
    println!("the answer is {}", square_of_sum - sum_of_squares);
}
