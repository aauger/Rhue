use std::{env::args, fs::File, io::{Read, self}, result::Result};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().skip(1).collect();
    let rule_file_path = &args[0];
    let program_file_path = &args[1];

    let mut rule_file = File::open(rule_file_path)?;
    let mut program_file = File::open(program_file_path)?;

    let mut rule_file_text = String::new();
    let mut program_file_text = String::new();
    
    let _ = rule_file.read_to_string(&mut rule_file_text);
    let _ = program_file.read_to_string(&mut program_file_text);

    println!("{}", rule_file_text);
    println!("{}", program_file_text);

    Ok(())
}

#[test]
fn test_tests() {
    assert_eq!(1, 1);
}
