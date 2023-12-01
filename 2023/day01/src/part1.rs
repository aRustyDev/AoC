use std::fs::read_to_string;
use regex::Regex;

fn main() {
    // static NUMBERS: Vec::<String> = Vec::<String>::new();
    const FILE_NAME: &str = "input/part1.txt";
    println!("The sum of the numbers is: {}", find_the_numbers(FILE_NAME).into_iter().sum::<i32>());
}

fn find_the_numbers(file_path: &str) -> Vec<i32> {
    read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(|s| vec![firstnum(s.to_string()), firstnum(s.chars().rev().collect::<String>())].join("").parse::<i32>().unwrap())
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
    #[cfg_attr(not(feature = "part1"), ignore)]
    fn find_the_numbers_test() {
        const FILE_NAME: &str = "input/part1_test.txt";
        let numvec: Vec<i32> = find_the_numbers(FILE_NAME);
        assert_eq!(vec![12, 38, 15, 77], numvec);
        assert_eq!(142, numvec.into_iter().sum());
    }

    #[test]
    #[cfg_attr(not(feature = "part1"), ignore)]
    fn firstnum_test() {
        let numstr: String = "Ru5t4Fun".to_string();
        let rtsmun: String = numstr.chars().rev().collect::<String>();
        assert_eq!("5".to_string(),firstnum(numstr));
        assert_eq!("4".to_string(),firstnum(rtsmun));
    }
}