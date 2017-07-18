fn make_english_below_20(x: u64) -> String {
    match x {
         0 => String::new(),
         1 => "one".to_string(),
         2 => "two".to_string(),
         3 => "three".to_string(),
         4 => "four".to_string(),
         5 => "five".to_string(),
         6 => "six".to_string(),
         7 => "seven".to_string(),
         8 => "eight".to_string(),
         9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
         _ => panic!("too large!"),
    }
}

fn make_english_below_100(x: u64) -> String {
    match x {
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
    0...19 => make_english_below_20(x),
         _ => [make_english_below_100(x / 10 * 10), make_english_below_20(x % 10)].join(" "),
    }
}

fn make_english(x: u64) -> String {
    if x > 1000 {
        panic!("too large!");
    } else if x == 1000 {
        "one thousand".to_string()
    } else if x % 100 == 0 {
        [make_english_below_20(x / 100), "hundred".to_string()].join(" ")
    } else if x < 100 {
        make_english_below_100(x)
    } else {
        [make_english_below_20(x / 100), "hundred and".to_string(), make_english_below_100(x % 100)].join(" ")
    }
}

fn main() {
    let mut answer = 0;
    for i in 1 .. 1001 {
        let stringized = make_english(i);
        let num_chars  = stringized.chars().filter(|&c| c != ' ').count();
        answer += num_chars;
//         println!("{} has {} charactors.", stringized, num_chars);
    }
    println!("the answer is {}", answer);
}
