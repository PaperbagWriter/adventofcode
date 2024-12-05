use std::isize;

use input::get_input;
mod input;

fn main() {
    let matrix: Vec<Vec<char>> = get_input().lines().map(|l| l.chars().collect()).collect();
    let _xmas_count = 0;
    let mas: Vec<char> = "XMAS".chars().collect();

    let mut count = 0;
    for i in 0..matrix.len() as isize {
        for j in 0..matrix[i as usize].len() as isize {
            let mut ress: Vec<bool> = vec![];
            if matrix[i as usize][j as usize] == mas[0] {
                ress.push(scan_dir1(&matrix, i, j, &mas));
                ress.push(scan_dir2(&matrix, i, j, &mas));
                ress.push(scan_dir3(&matrix, i, j, &mas));
                ress.push(scan_dir4(&matrix, i, j, &mas));
                ress.push(scan_dir5(&matrix, i, j, &mas));
                ress.push(scan_dir6(&matrix, i, j, &mas));
                ress.push(scan_dir7(&matrix, i, j, &mas));
                ress.push(scan_dir8(&matrix, i, j, &mas));
                for ele in ress {
                    if ele {
                        count += 1
                    }
                }
            }
        }
    }
    println!("{}", count)
}

fn scan_dir1(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i + k, j, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir2(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i - k, j, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir3(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i, j + k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir4(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i, j - k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir5(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i + k, j + k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir6(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i - k, j - k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir7(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i + k, j - k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}
fn scan_dir8(matrix: &Vec<Vec<char>>, i: isize, j: isize, mas: &Vec<char>) -> bool {
    for k in 1..mas.len() as isize {
        let current_char_scan = mas[k as usize];
        let res = safe_check_matrix_pos(&matrix, i - k, j + k, current_char_scan);
        if !res {
            return false;
        }
    }
    return true;
}

fn safe_check_matrix_pos(matrix: &Vec<Vec<char>>, i: isize, j: isize, char: char) -> bool {
    if i >= 0 && i < matrix.len() as isize && j >= 0 && j < matrix[i as usize].len() as isize {
        return char == matrix[i as usize][j as usize];
    }
    return false;
}
