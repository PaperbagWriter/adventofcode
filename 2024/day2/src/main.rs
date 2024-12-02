use std::i32;

mod input;
fn main() {
    let input = input::get_input();
    let x: Vec<&str> = input.lines().collect();
    let mut safe_lines = 0;
    for i in 0..x.len() {
        let separated: Vec<i32> = x[i]
            .split(" ")
            .map(|x| x.parse().expect("could not parse"))
            .collect();

        let initial_delta_is_positive = (separated[1] - separated[0]).is_positive();
        for j in 1..separated.len() {
            let delta = separated[j] - separated[j - 1];
            let less_than_three_delta = delta.abs() <= 3;
            let delta_is_positive = delta.is_positive();
            let delta_same_sign_than_initial_delta = initial_delta_is_positive == delta_is_positive;
            let delta_non_zero = delta != 0;

            if delta_same_sign_than_initial_delta && delta_non_zero && less_than_three_delta {
                if j == separated.len() - 1 {
                    safe_lines += 1
                }
            } else {
                break;
            }
        }
    }
    assert!(safe_lines == 359);
    print!("{}", safe_lines)
}
