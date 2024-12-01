use std::{collections::HashMap, i32};
mod input;

fn get_both_lists_sorted() -> (Vec<i32>, Vec<i32>) {
    let input = input::get_input();
    let list = input.trim().replace("   ", ",").replace("\n", ",");
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];
    let list: Vec<&str> = list.split(',').collect();
    for i in 0..list.len() {
        match list.get(i) {
            Some(&str) => {
                if i % 2 == 0 {
                    list_one.push(str.parse().expect("error"))
                } else {
                    list_two.push(str.parse().expect("error"))
                }
            }
            None => todo!(),
        }
    }
    list_one.sort();
    list_two.sort();
    (list_one, list_two)
}

fn part1() {
    let (list_one, list_two) = get_both_lists_sorted();
    let mut distances = 0;
    for i in 0..list_one.len() {
        let distance = list_one.get(i).expect("whatever") - list_two.get(i).expect("whatever");
        let distance = distance.abs();
        distances += distance;
    }
    println!("Part 1: {}", distances);
}

fn part2() {
    let (list_one, list_two) = get_both_lists_sorted();
    let mut list_2_counts: HashMap<i32, i32> = HashMap::new();
    for i in 0..list_two.len() {
        let key = list_two.get(i).expect("error");
        match list_2_counts.get(key) {
            Some(&val) => {
                let new_value = val + 1;
                list_2_counts.insert(*key, new_value);
            }
            None => {
                list_2_counts.insert(*key, 1);
            }
        };
    }
    let mut similarity_score = 0;
    for i in 0..list_one.len() {
        let entry = list_one.get(i).expect("error");
        match list_2_counts.get(entry) {
            Some(&val) => {
                let local_sim_score = val * entry;
                similarity_score += local_sim_score;
            }
            None => {}
        }
    }
    println!("Part 2: {}", similarity_score);
}

fn main() {
    part1();
    part2();
}
