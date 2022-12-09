use std::io;
use std::fs::File;

pub fn count_calories(file_lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut currelf_calories: i32 = 0;
    let mut elf_calories = Vec::new();
    for line in file_lines {
        let temp_line = line.unwrap();
        if temp_line.trim().is_empty() {
            elf_calories.push(currelf_calories);
            currelf_calories = 0;
            continue; 
        }
        println!("{temp_line}");
        currelf_calories += temp_line.parse::<i32>().unwrap();
    }
    elf_calories
}
