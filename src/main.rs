mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn get_input_data(file_name: &str) -> Vec<i64> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut numbers:Vec<i64> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        debug!("{:?}", line);
        numbers.push(line.parse::<i64>().unwrap());
    }
    return numbers;
}

fn is_sum_of_two_element_in_array(numbers: &[i64], goal: i64) -> bool {
    for n in numbers {
        if &(goal - n) == n {
            continue
        }
        if numbers.contains(&(goal - n)) {
            return true;
        }
    }
    return false
}

fn solution_part_1(file_name: &str, window_size: usize) -> i64 {
    let numbers: Vec<i64> = get_input_data(file_name);
    let mut window: &[i64];

    for n in 0..numbers.len() - window_size {
        debug!("{:?}", n);
        window = &numbers[n..window_size+n];
        debug!("{:?}", window);
        if !is_sum_of_two_element_in_array(window, numbers[n+window_size]) {
            return numbers[n+window_size];
        }
    }
    return 0
}

fn solution_part_2(file_name: &str, window_size: usize) -> i64 {
    let numbers: Vec<i64> = get_input_data(file_name);
    let goal = solution_part_1(file_name, window_size);
    let mut window: &[i64];
    debug!("Länge: {}, Numbers: {:?}", numbers.len(), numbers);
    for n in 2..numbers.len()+1 {
        for m in 0..numbers.len()+1-n {
            window = &numbers[m .. m + n];
            debug!("Start: {}, End: {}, Window: {:?}", m, m + n, window);
            if window.iter().sum::<i64>() == goal {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }
    return 0
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt", 25));
    info!("{:?}", solution_part_2("inputData.txt", 25));
}
