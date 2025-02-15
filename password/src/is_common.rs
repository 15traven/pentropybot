use std::{
    io::{BufRead, BufReader}, 
    collections::HashSet,
    fs::File
};

pub fn is_common(password: &str) -> bool {
    let rockyou = File::open("password/data/rockyou.txt").expect("Failed to open rockyou.txt");
    let reader = BufReader::new(rockyou);
    let common_passwords = reader
        .lines() // <-- This method exists and works!
        .filter_map(Result::ok) // Filters out errors and unwraps successful lines
        .collect::<HashSet<String>>();

    common_passwords.contains(password)
}