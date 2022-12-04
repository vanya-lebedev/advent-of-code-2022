use std::fs;

pub fn solve() {
    let file_name = r"src\task01b\input.txt";

    let segments = get_segments(file_name);

    let mut elf_calories: Vec<i32> = segments.iter().map(| segment | get_calories(segment)).collect();
    elf_calories.sort();
    
    let sorted_calories: Vec<i32> = elf_calories.iter().rev().enumerate()
        .take_while(|&(index, &_n)| index < 3)
        .map(|(_, &n)| n).collect();

    let max: i32 = sorted_calories.iter().sum();

    print!("{max}");
}

fn get_calories(elf_pack: &Vec<String>) -> i32 {
    let calories: Vec<i32> = elf_pack.iter().map(| x | x.parse::<i32>().unwrap()).collect();
    return calories.iter().sum();
}

fn get_segments(file_name: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(file_name).unwrap();
    let lines = contents.lines();

    let mut result: Vec<Vec<String>> = Vec::new();
    let mut aggregator: Vec<String> = Vec::new();

    for line in lines {
        if line == "" {
            result.push(aggregator);
            aggregator = Vec::new();
        }
        else {
            aggregator.push(line.to_string());
        }
    }

    result.push(aggregator);

    return result;
}