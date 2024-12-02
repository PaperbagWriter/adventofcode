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

        for j in 1..separated.len() {
            if separated[j] > separated[j - 1] && (separated[j] - separated[j - 1]).abs() <= 3 {
                if j == separated.len() - 1 {
                    safe_lines += 1
                }
            } else {
                break;
            }
        }
        for j in 1..separated.len() {
            if separated[j] < separated[j - 1] && (separated[j] - separated[j - 1]).abs() <= 3 {
                if j == separated.len() - 1 {
                    safe_lines += 1
                }
            } else {
                break;
            }
        }
    }
    print!("{}", safe_lines)
}
