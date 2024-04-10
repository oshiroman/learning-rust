fn main() {
    let guess = "614".parse::<i32>().unwrap();

    let x = 3.0;
    let y: f32 = 2.7182818;

    // addition
    let sum = 2 + 3;
    println!("[sum] {sum}");

    // subtraction
    let diff = 1.732 - 2.236;
    println!("[diff] {diff}");

    // multiplication
    let product = 12345679 * 23;
    println!("[product] {product}");

    // division
    let quotient = 58.0 / 173.1;
    let truncated = -2401 / 2443;
    println!("[quotient] {quotient}");
    println!("[truncated] {truncated}");

    // remainder
    let remainder = 341 % 17;
    println!("[remainder] {remainder}");
}

#[test]
fn test() {
    let guess: u32 = "614".parse().expect("Not a number!");
    assert_eq!(guess, 614)
}
