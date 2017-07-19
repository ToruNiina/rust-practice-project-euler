extern crate num;
use num::rational::Ratio;

fn cancel_digit(x:u32, y:u32) -> Vec<(u32, u32)> {
    let x_str = x.to_string();
    let y_str = y.to_string();
    let mut retval: Vec<(u32, u32)> = Vec::new();
    if x_str.chars().nth(0).unwrap() == y_str.chars().nth(0).unwrap() {
        retval.push((x_str.chars().nth(1).unwrap().to_digit(10).unwrap(), y_str.chars().nth(1).unwrap().to_digit(10).unwrap()));
    } else if x_str.chars().nth(0).unwrap() == y_str.chars().nth(1).unwrap() {
        retval.push((x_str.chars().nth(1).unwrap().to_digit(10).unwrap(), y_str.chars().nth(0).unwrap().to_digit(10).unwrap()));
    } else if x_str.chars().nth(1).unwrap() == y_str.chars().nth(0).unwrap() {
        retval.push((x_str.chars().nth(0).unwrap().to_digit(10).unwrap(), y_str.chars().nth(1).unwrap().to_digit(10).unwrap()));
    } else if x_str.chars().nth(1).unwrap() == y_str.chars().nth(1).unwrap() {
        retval.push((x_str.chars().nth(0).unwrap().to_digit(10).unwrap(), y_str.chars().nth(0).unwrap().to_digit(10).unwrap()));
    }
    retval
}

fn is_digit_cancelling_fractions(x: u32, y:u32) -> bool {
    let canceled = cancel_digit(x, y);
    for (numer, denom) in canceled {
        if denom == 0 { continue; }

        let ccl = Ratio::new(numer, denom);
        let ord = Ratio::new(x, y);
        if ccl == ord {return true;}
    }
    false
}

fn main() {

    println!("{}/{} is digit_cancelling_fractions? : {}", 30, 50, is_digit_cancelling_fractions(30, 50));

    let mut dcfracs: Vec<Ratio<u32>> = Vec::new();
    for d in 11..100 {
        for n in 10..d {
            if n % 10 == 0 && d % 10 == 0 { continue; }

            if is_digit_cancelling_fractions(n, d) {
                println!("{}/{}", n , d);
                dcfracs.push(Ratio::new(n, d));
            }
        }
    }
    let product = dcfracs.iter().fold(Ratio::new(1,1), |p, q| p * q);
    println!("{:?}", product);


}
