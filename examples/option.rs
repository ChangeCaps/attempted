use attempted::attempt;

#[attempt]
fn test(input: Option<i32>) -> i32 {
    input?
}

fn main() {
    assert_eq!(test(Some(42069)), 42069);
    assert_eq!(test(None), 0);
}
