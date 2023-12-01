use std::fs::read_to_string;
use regex::Regex;

fn main() {
    // static NUMBERS: Vec::<String> = Vec::<String>::new();
    const FILE_NAME: &str = "input/part1.txt";
    find_the_numbers(FILE_NAME);
}

fn find_the_numbers(file_path: &str) -> Vec<String> {
    read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(|s| vec![firstnum(s.to_string()), firstnum(s.chars().rev().collect::<String>())].join(""))
        .collect()
}

fn firstnum(line: String) -> String {
    let re: Regex = Regex::new("[0-9]").unwrap();
    re.find(&line).unwrap().as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_firstnum() {
        let numstr: String = "Ru5t4Fun".to_string();
        let rtsmun: String = numstr.chars().rev().collect::<String>();
        assert_eq!("5".to_string(),firstnum(numstr));
        assert_eq!("4".to_string(),firstnum(rtsmun));
    }
}