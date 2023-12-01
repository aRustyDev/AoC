use std::env;
use std::fs;

fn main() {
    find_the_numbers("input/part1.txt");
}

fn find_the_numbers(file_path) {

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_opens() {
        find_the_numbers("input/part1.txt");
        assert_eq!(2 + 2, 4);
    }
}