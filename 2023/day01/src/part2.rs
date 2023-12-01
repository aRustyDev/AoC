use std::fs::read_to_string;
use regex::Regex;

fn main() {
    const FILE_NAME: &str = "input/part2.txt";
    println!("The sum of the numbers is: {}", find_the_numbers(FILE_NAME).into_iter().sum::<i32>());
}

fn find_the_numbers(file_path: &str) -> Vec<i32> {
    read_to_string(file_path) 
        .unwrap()
        .lines()
        .map(|s| vec![firstnum(s.to_string(), false), firstnum(s.chars().rev().collect::<String>(), true)].join("").parse::<i32>().unwrap())
        .collect()
}

fn firstnum(line: String, reverse: bool) -> String {
    let re: Regex = Regex::new("([0-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero)").unwrap();
    let re2: Regex = Regex::new("([0-9])|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|(orez)").unwrap();
    if reverse {
        match re2.find(&line).unwrap().as_str() {
            "eno" => "1".to_string(),
            "owt" => "2".to_string(),
            "eerht" => "3".to_string(),
            "ruof" => "4".to_string(),
            "evif" => "5".to_string(),
            "xis" => "6".to_string(),
            "neves" => "7".to_string(),
            "thgie" => "8".to_string(),
            "enin" => "9".to_string(),
            "orez" => "0".to_string(),
            _ => re2.find(&line).unwrap().as_str().to_string(),
        }
    } else {
        match re.find(&line).unwrap().as_str() {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            "zero" => "0".to_string(),
            _ => re.find(&line).unwrap().as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "part2"), ignore)]
    fn find_the_numbers_test() {
        const FILE_NAME: &str = "input/part2_test.txt";
        let numvec: Vec<i32> = find_the_numbers(FILE_NAME);
        assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], numvec);
        assert_eq!(281, numvec.into_iter().sum());
    }

    #[test]
    #[cfg_attr(not(feature = "part2"), ignore)]
    fn firstnum_test() {
        let numstr: String = "Ru5t4Fun".to_string();
        let rtsmun: String = numstr.chars().rev().collect::<String>();
        assert_eq!("5".to_string(),firstnum(numstr, false));
        assert_eq!("4".to_string(),firstnum(rtsmun, true));
    }
}