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

    // boolean
    let t = true;
    let f: bool = false;
    println!("[true] {t} [false] {f}");

    // character
    let c = 'c';
    let r = 'ℝ';
    let zzz = '💤';
    println!("{c} {r} {zzz}");

    // tuple
    let tuple: (i32, f64, i8) = (2119, 9.8, -3);
    let (a, b, c) = tuple;
    println!("a: {a}, b: {b}, c: {c}");

    let tup = ("PERFECT", 100, false);
    let perfect = tup.0;
    let one_hundred = tup.1;
    let f = tup.2;
    println!("[perfect] {perfect} [one_hundred] {one_hundred} [f] {f}");

    // array
    let directions = ["North", "East", "South", "West"];
    let east = directions[1];
    println!("[1] {east}");

    let all_u64: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let all_zero = [0; 10];
    println!("{all_u64:?}");
    println!("{all_zero:?}");

    another_function(901);
    another_function2(2401);

    println!("{}", add(1834, 1905));
}

fn another_function(number: i32) {
    println!("selected: {number}");
}

fn another_function2(number: i32) {
    let y = {
        let x = 20;
        x + number
    };

    println!("y is {y}");
}

fn add(x: i32, y: i32) -> i32 {
    return x + y
}

#[test]
fn test() {
    let guess: u32 = "614".parse().expect("Not a number!");
    assert_eq!(guess, 614)
}
