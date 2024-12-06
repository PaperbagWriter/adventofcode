use std::{cmp::Ordering, collections::HashMap};
mod input;

fn main() {
    let _test_input = String::from(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    let input = input::get_input();
    let mut sum_rightly_ordered = 0;
    let mut sum_wrong_ordered = 0;
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let (order_rules, updates) = input.split_once("\n\n").unwrap();
    for line in order_rules.lines() {
        let (left, right) = line.split_once('|').unwrap();
        let key = left.parse().unwrap();
        let value = right.parse().unwrap();
        match hash.get(&key) {
            Some(item) => {
                let mut item = item.clone();
                item.push(value);
                hash.insert(key, item);
            }
            None => {
                hash.insert(key, [value].to_vec());
            }
        }
    }
    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    for i in 0..updates.len() {
        let update_in_order = is_in_order(updates.get(i).unwrap().to_vec(), &hash);
        let middle_index = middle_index(&updates.get(i).unwrap().to_vec());
        if update_in_order {
            sum_rightly_ordered += updates.get(i).unwrap().to_vec()[middle_index];
        } else {
            let ordered = order(updates.get(i).unwrap().to_vec(), &hash);
            sum_wrong_ordered += ordered[middle_index];
        }
    }
    println!("{}", sum_rightly_ordered);
    println!("{}", sum_wrong_ordered);
}

fn order(array: Vec<i32>, hash: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut clone = array.clone();
    clone.sort_by(|a, b| {
        if hash.get(&a).is_some() {
            if hash.get(&a).unwrap().contains(b) {
                return Ordering::Less;
            }
        }
        return Ordering::Greater;
    });
    return clone;
}

fn middle_index(arr: &Vec<i32>) -> usize {
    (arr.len() - 1) / 2
}
fn is_in_order(array: Vec<i32>, hash: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..array.len() {
        if hash.get(&array[i]).is_some() {
            let order_rules = hash.get(&array[i]).unwrap();
            let mut j = i;
            while j > 0 {
                for l in order_rules {
                    if array[j - 1] == *l {
                        return false;
                    }
                }
                j -= 1;
            }
        }
    }
    return true;
}
