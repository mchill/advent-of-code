use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

pub fn get_day(filepath: &str) -> String {
    return filepath.rsplit("/").nth(1).unwrap().to_owned();
}

pub fn read_file(day: String, filename: &str) -> String {
    let mut file = get_file(day, filename);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    return input;
}

pub fn read_lines(day: String, filename: &str) -> Vec<String> {
    let file = get_file(day, filename);
    return BufReader::new(file).lines().map(|line| line.unwrap()).collect();
}

pub fn add_arrays<T: std::fmt::Debug + Add, const COUNT: usize>(a: [T; COUNT], b: [T; COUNT]) -> [T; COUNT]
where
    T: Clone,
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    return a.into_iter().zip(b.clone()).map(|(a, b)| a + b).collect::<Vec<T>>().try_into().unwrap();
}

pub fn subtract_arrays<T: std::fmt::Debug + Sub, const COUNT: usize>(a: [T; COUNT], b: [T; COUNT]) -> [T; COUNT]
where
    T: Clone,
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    return a.into_iter().zip(b.clone()).map(|(a, b)| a - b).collect::<Vec<T>>().try_into().unwrap();
}

fn get_file(day: String, filename: &str) -> File {
    let filepath = format!("src/bin/{}/{}", day, filename);
    return File::open(filepath).unwrap();
}
