use std::i32;
mod input;

fn check_line_safety(separated: &Vec<i32>) -> bool {
    let initial_delta_is_positive = (separated[1] - separated[0]).is_positive();
    for j in 1..separated.len() {
        let delta = separated[j] - separated[j - 1];
        let less_than_three_delta = delta.abs() <= 3;
        let delta_is_positive = delta.is_positive();
        let delta_same_sign_than_initial_delta = initial_delta_is_positive == delta_is_positive;
        let delta_non_zero = delta != 0;

        if delta_same_sign_than_initial_delta && delta_non_zero && less_than_three_delta {
            if j == separated.len() - 1 {
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}

fn day2(dampener: bool) {
    let input = input::get_input();
    let x: Vec<&str> = input.lines().collect();
    let mut safe_lines = 0;
    for i in 0..x.len() {
        let separated: Vec<i32> = x[i]
            .split(" ")
            .map(|x| x.parse().expect("could not parse"))
            .collect();

        if check_line_safety(&separated) {
            safe_lines += 1;
        } else if dampener {
            for i in 0..separated.len() {
                let mut new_array = separated.clone();
                new_array.remove(i);
                if check_line_safety(&new_array) {
                    safe_lines += 1;
                    break;
                }
            }
        };
    }
    println!("{}", safe_lines)
}

fn main() {
    day2(false);
    day2(true);
}
