pub mod day1;

pub mod file_io {
    use std::io;
    use std::fs::File;
    use std::path::Path;
    use std::io::BufRead;
    pub fn read_lines<P>(filename: P) -> io::Result::<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
    }
    pub fn read_file(fname: &str) -> io::Lines<io::BufReader<File>> {
        match read_lines(fname) {
            Ok(lines) => lines,
            Err(error) => panic!("Something went wrong: {error:?}"),
        }
    }
}
