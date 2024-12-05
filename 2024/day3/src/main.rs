mod input;

fn main() {
    let input = input::get_input();
    part1(&input);
    part2(&input);
}

fn sum_multiplications(input: &str) -> i32 {
    let mut result: i32 = 0;

    let splitted: Vec<&str> = input.split("mul(").collect();
    for i in 0..splitted.len() {
        let sub = splitted[i];
        let sub: Vec<&str> = sub
            .split_terminator(')')
            .filter(|x| x.contains(','))
            .collect();
        for i in 0..sub.len() {
            let (str1, str2) = match sub[i].split_once(',') {
                Some(a) => (a.0.parse::<i32>(), a.1.parse::<i32>()),
                None => todo!(),
            };
            match (str1, str2) {
                (Ok(str1), Ok(str2)) => {
                    result += str1 * str2;
                }
                _ => {}
            }
        }
    }

    return result;
}

fn part1(input: &str) {
    let result = sum_multiplications(input);
    println!("{}", result);
}

fn part2(input: &str) {
    let mut result = 0;
    let mut input = input;
    while input.contains("don't()") {
        let (subs, rest) = input.split_at(input.find("don't()").unwrap());
        result += sum_multiplications(subs);
        let (_, rest) = rest.split_at(rest.find("do()").unwrap() + 4);
        input = rest;
    }
    result += sum_multiplications(input);
    println!("{}", result)
}
