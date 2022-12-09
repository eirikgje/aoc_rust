fn main() {
    use aoc::file_io;
    use aoc::day1;

    let fname = String::from("/home/eirik/src/aoc/2022/input_day1");

    let file_lines = file_io::read_file(&fname);
    let elf_calories = day1::count_calories(file_lines);
    if let Some(max_val) = elf_calories.iter().max() {
        println!("{max_val}");
    }
}

