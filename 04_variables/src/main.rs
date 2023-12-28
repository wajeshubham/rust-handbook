// NOTE: Compilation error:
// fn main() {
//     let x = 5;
//     println!("You entered {x}");
//     x = 6;
//     println!("You changed x to {x}");
// }

// NOTE: Variable mutability in Rust
// fn main() {
//     // We’re allowed to change the value bound to x from 5 to 6 when mut is used.
//     let mut x = 5;
//     println!("You entered {x}");
//     x = 6;
//     println!("You changed x to {x}");
// }

// NOTE: Variable shadowing in Rust
// NOTE: This kind of work similar to javascript
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}"); // 12
//     }

//     println!("The value of x is: {x}"); // 6
// }

// NOTE: Using let we can create two different variables with different type with same name
// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();

//     println!("There are {spaces} in the string") // prints: There are 3 in the string
// }

// NOTE: if we use mut and try to do the above thing on same variable it throws an error
// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len(); // Throws: error[E0308]: mismatched types

//     println!("There are {spaces} in the string");
// }

fn main() {}
