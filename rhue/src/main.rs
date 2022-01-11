use std::{fs::File, env::args, io::Read};

fn main() {
    let args: Vec<String> = args().collect();
    let file = File::open(&args[0]);
    let mut buf = String::new();
    let value = match file {
        Ok(mut s_file) => s_file.read_to_string(&mut buf),
        Err(_) => panic!("goodbye!"),
    };
    //value.ok().
}

#[test]
fn test_tests() {
    assert_eq!(1, 1);
}
