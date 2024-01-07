// NOTE: Integer type
// fn main() {
//     let number: u8 = "36".parse().expect("Not a number!");
// }

// NOTE: floating type number
// fn main() {
//     let x = 2.05; // f64

//     let y: f32 = 3.05; // f32

//     println!("Value of x is {x}"); // Value of x is 2.05
//     println!("Value of y is {y}"); // Value of y is 3.05
// }

// NOTE: numeric operations
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2; // This returns f64 (64 bits floating number)
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;

//     println!("{sum}"); // 15
//     println!("{difference}"); // 91.2
//     println!("{product}"); // 120
//     println!("{quotient}"); // 1.7608695652173911 (64 bits)
//     println!("{truncated}"); // -1
//     println!("{remainder}"); // 3
// }

// NOTE: Booleans

// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// NOTE: character type
// fn print_type_of<T>(_: &T, var: &str) {
//     println!("Type of {var} is {}", std::any::type_name::<T>())
// }

// fn main() {
//     let c = 'z'; // char is always specified with single quotes, string literals are specified in double quotes
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     let double_quotes = "s";

//     print_type_of(&c, "c");
//     print_type_of(&z, "z");
//     print_type_of(&heart_eyed_cat, "heart_eyed_cat");
//     print_type_of(&double_quotes, "double_quotes");
// }

// NOTE: Tuple Type
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1); // explicit type annotation which is optional
// }

// NOTE: accessing tuple values using .
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let _five_hundred = x.0;

//     let _six_point_four = x.1;

//     let _one = x.2;
// }

// NOTE: Arrays in rust
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// NOTE: access array elements
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     println!("{}", a[1]) // 2
// }

// NOTE: access array elements at index greater than available
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     println!("{}", a[10]) // runtime error
// }

fn main() {}
