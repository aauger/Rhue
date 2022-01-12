use std::{env::args, fs::File, io::Read};

fn main() {
    let args: Vec<String> = args().collect();
    println!("{}", &args[1]);
    let file = File::open(&args[1]);
    let mut buf = String::new();
    let _ = match file {
        Ok(mut s_file) => s_file.read_to_string(&mut buf),
        Err(_) => panic!("goodbye!")
    };
    println!("{}", buf);
}

#[test]
fn test_tests() {
    assert_eq!(1, 1);
}
