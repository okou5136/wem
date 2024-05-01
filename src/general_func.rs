use anyhow::Context;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn does_contain_vec(target: String, chars: Vec<char>) -> bool
{
    for tarchar in target.chars() {
        for single_char in &chars {
            if tarchar == *single_char {
                return true;
            }
        }
    }
    return false;
}

pub fn does_contain_string(target: &Vec<String>, string: String) -> bool {
    for tarstr in target.into_iter() {
        if *tarstr == string {
            return true;
        }
    }
    return false;
}


pub fn read_file<P>(file_path: P) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file =  File::open(file_path).with_context(|| format!("failed to open the file"))?;
    Ok(io::BufReader::new(file).lines())
}
