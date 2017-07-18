fn is_leap(x:u64) -> bool {
    x % 400 == 0 || (x % 100 != 0 && x % 4 == 0)
}

fn num_days_in_year(y:u64) -> u64 {
    if is_leap(y) {366} else {365}
}

fn num_days_in_month(y: u64, m:u64) -> u64 {
    match m {
        1 => 31,
        2 => if is_leap(y) {29} else {28},
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
       10 => 31,
       11 => 30,
       12 => 31,
       _  => panic!("invalid month"),
    }
}

fn calc_total_days_until(year: u64, month: u64) -> u64 {
    (1901..year).map(|y| num_days_in_year(y)).sum::<u64>() +
    (1..month).map(|m| num_days_in_month(year, m)).sum::<u64>()
}

fn is_first_day_sunday(year: u64, month: u64) -> bool {
    let total_days = 1 + calc_total_days_until(year, month);
    (total_days+1) % 7 == 0
}

fn main() {
    let mut sum: u64 = 0;
    for i in 1901..2001 {
        for j in 1..13 {
            if is_first_day_sunday(i, j) {sum += 1;}
        }
    }
    println!("{}", sum);
}
