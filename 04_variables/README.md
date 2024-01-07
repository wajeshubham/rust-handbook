# Variables in Rust

By default, variables are immutable in Rust. This is one of many advantages Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable.

# Errors while mutating the immutable

open `src/main.rs` and replace its code with the following code, which won’t compile just yet:

Filename: `src/main.rs`

FOllowing code does not compile!

```rust
fn main() {
    let x = 5;
    println!("You entered {x}");
    x = 6;
    println!("You changed x to {x}");
}
```

Save and run the program using `cargo run`. You should receive an error message regarding an immutability error, as shown in this output:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("You entered {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to previous error
```

# Allow mutability

Mutability can be very useful, and can make code more convenient to write. Although variables are immutable by default, you can make them mutable by adding `mut` in front of the variable name.

Adding `mut` also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

For example, let’s change `src/main.rs` to the following:

```rust
fn main() {
    let mut x = 5;
    println!("You entered {x}");
    x = 6;
    println!("You changed x to {x}");
}
```

When we run the program now, we get this:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 4.75s
     Running `target/debug/variables`
You entered 5
You changed x to 6
```

# Constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re **always immutable**. You declare constants using the co``nst keyword instead of the `let` keyword, and **the type of the value must be annotated.**

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

Here’s an example of a constant declaration:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

_Rust’s naming convention for constants is to use all uppercase with underscores between words._

Constants are valid for the entire time a program runs, within the scope in which they were declared. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about.

# Variable Shadowing

In Rust, you can declare a new variable with the same name as a previous variable. We call it, the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. (Work kind of similar to JavaScript)

In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // prints 12
        // After the scope ends x returns the value thats in the outer scope
    }

    println!("The value of x is: {x}"); // prints 6
}
```

This program first binds `x` to a value of `5`. Then it creates a new variable `x` by repeating `let x =`, taking the original value and adding `1` so the value of `x` is then `6`.

Then, within an inner scope created with the curly brackets, the third let statement also shadows `x` and creates a new variable, multiplying the previous value by `2` to give `x` a value of `12`. **When that scope is over, the inner shadowing ends and `x` returns to being `6`.**

When we run this program, it will output the following:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

# Variable Shadowing in mutable variable `mut`

Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

**The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.**

For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```rust
  let spaces = "   ";
  let spaces = spaces.len(); // This works as we are creating new var using let keyword
```

The first `spaces` variable is a `string` type and the second `spaces` variable is a `number` type.

Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; **instead, we can reuse the simpler `spaces` name.**

However, if we try to use `mut` for this, as shown here, we’ll get a compile-time error:

```rust
  let mut spaces = "   ";
  spaces = spaces.len(); // throw error, because we are changing type of the same variable. Using mute we can change the value but cannot change the value's type
```

The error says we’re not allowed to mutate a variable’s type:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```
