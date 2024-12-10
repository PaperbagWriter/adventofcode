use std::{collections::HashSet, isize};

use input::get_input;
mod input;

fn main() {
    let matrix: Vec<Vec<char>> = get_input().lines().map(|l| l.chars().collect()).collect();
    let mut hash: HashSet<(isize, isize)> = HashSet::new();
    let guard_inital_position = find_initial_position(&matrix, '^');
    hash.insert(guard_inital_position);
    let north = (-1, 0);
    let east = (0, 1);
    let south = (1, 0);
    let west = (0, -1);
    let directions = [north, east, south, west];
    let part1_solution = part1(&matrix, &mut hash, directions, guard_inital_position);
    println!("Part 1: {}", part1_solution);
    let part2_solution = part2(&matrix, directions, guard_inital_position);
    println!("Part 2: {}", part2_solution);
}

fn part2(
    matrix: &Vec<Vec<char>>,
    directions: [(isize, isize); 4],
    init_pos: (isize, isize),
) -> i32 {
    let matrices = create_different_matrices(matrix);
    let mut count = 0;
    for i in 0..matrices.len() {
        print!("\rProcessing {}/{}", i, matrices.len() - 1);
        let mut hash: HashSet<(isize, isize)> = HashSet::new();
        let mut edges: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
        hash.insert(init_pos);
        let res = find_edges(&matrices[i], &mut hash, &mut edges, directions, init_pos);
        if res {
            count += 1;
        }
    }
    println!("{}", "");
    return count;
}

fn create_different_matrices(matrix: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut matrics: Vec<Vec<Vec<char>>> = vec![];
    for i in 0..matrix.len() as isize {
        for j in 0..matrix[i as usize].len() as isize {
            let is_not_block = safe_check_matrix_pos(matrix, i, j, '.');
            if is_not_block {
                let mut copy = matrix.clone();
                copy[i as usize][j as usize] = '#';
                matrics.push(copy);
            }
        }
    }
    return matrics;
}

fn find_edges(
    matrix: &Vec<Vec<char>>,
    hash: &mut HashSet<(isize, isize)>,
    edges: &mut HashSet<((isize, isize), (isize, isize))>,
    directions: [(isize, isize); 4],
    init_pos: (isize, isize),
) -> bool {
    let mut current_direction = 0;
    let mut i = init_pos.0;
    let mut j = init_pos.1;
    let mut start_edge = init_pos.clone();
    loop {
        let (modifier_i, modifier_j) = directions[current_direction];
        i += modifier_i;
        j += modifier_j;
        let in_bound = safe_check_matrix_char(matrix, i, j);
        if in_bound.is_none() {
            return false;
        } else {
            let char = in_bound.unwrap();
            if char == '.' {
                hash.insert((i, j));
            } else if char == '#' {
                if current_direction == directions.len() - 1 {
                    current_direction = 0;
                } else {
                    current_direction += 1;
                }

                i -= modifier_i;
                j -= modifier_j;
                let key = (start_edge, (i, j));
                match edges.get(&key) {
                    Some(_) => return true,
                    None => {
                        edges.insert(key);
                        start_edge = (i, j);
                    }
                }
            }
        }
    }
}

fn part1(
    matrix: &Vec<Vec<char>>,
    hash: &mut HashSet<(isize, isize)>,
    directions: [(isize, isize); 4],
    init_pos: (isize, isize),
) -> usize {
    let mut current_direction = 0;
    let mut i = init_pos.0;
    let mut j = init_pos.1;
    loop {
        let (modifier_i, modifier_j) = directions[current_direction];
        i += modifier_i;
        j += modifier_j;
        let in_bound = safe_check_matrix_char(matrix, i, j);
        if in_bound.is_none() {
            break;
        } else {
            let char = in_bound.unwrap();
            if char == '.' {
                hash.insert((i, j));
            } else if char == '#' {
                if current_direction == directions.len() - 1 {
                    current_direction = 0;
                } else {
                    current_direction += 1;
                }

                i -= modifier_i;
                j -= modifier_j;
            }
        }
    }
    return hash.len();
}

fn find_initial_position(matrix: &Vec<Vec<char>>, char: char) -> (isize, isize) {
    for i in 0..matrix.len() as isize {
        for j in 0..matrix[i as usize].len() as isize {
            let char_at_pos = safe_check_matrix_char(matrix, i, j);
            if char_at_pos.unwrap() == char {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn safe_check_matrix_pos(matrix: &Vec<Vec<char>>, i: isize, j: isize, char: char) -> bool {
    if i >= 0 && i < matrix.len() as isize && j >= 0 && j < matrix[i as usize].len() as isize {
        return char == matrix[i as usize][j as usize];
    }
    return false;
}
fn safe_check_matrix_char(matrix: &Vec<Vec<char>>, i: isize, j: isize) -> Option<char> {
    if i >= 0 && i < matrix.len() as isize && j >= 0 && j < matrix[i as usize].len() as isize {
        return Some(matrix[i as usize][j as usize]);
    }
    return None;
}
