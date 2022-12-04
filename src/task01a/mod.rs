use std::fs;

pub fn solve() {
    let file_name = r"src\task01a\input.txt";

    let segments = get_segments(file_name);

    let elf_calories: Vec<i32> = segments.iter().map(| segment | get_calories(segment)).collect();
    let max = elf_calories.iter().max().unwrap(); 
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