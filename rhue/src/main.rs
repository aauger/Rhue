fn main() {
    let mut input: String = String::from("hello");
    println!("{}", input);
    input += " world!";
    println!("{}", input);
}

#[test]
fn test_tests() {
    assert_eq!(1, 1);
}
