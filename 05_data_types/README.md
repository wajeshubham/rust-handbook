# Data types in Rust

Every value in Rust is of a certain _data type_, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: `scalar` and `compound`.

Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as converting a `String` to a `numeric` type using `parse`, like this:

```rust
let number: u32 = "36".parse().expect("Not a number!");
```

If we don’t add the `: u32` type annotation shown in the preceding code, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

```bash
   Compiling data_types v0.1.0 (file:///projects/05_data_types)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let number = "36".parse().expect("Not a number!");
  |         ^^^^^^
  |
help: consider giving `number` an explicit type
  |
2 |     let number: /* Type */ = "36".parse().expect("Not a number!");
  |               ++++++++++++

For more information about this error, try `rustc --explain E0282`.
error: could not compile `data_types` (bin "data_types") due to previous error
```

# Scalar Types

A scalar type represents a **single value**.

Rust has four primary scalar types:

- integers
- floating-point numbers
- Booleans
- characters

You may recognize these from other programming languages. Let’s jump into how they work in Rust.

# Integers

An `integer` is a number without a **fractional component**. For example, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (_signed integer types start with i instead of u_) that takes up `32 bits` of space.

Following table shows shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Each variant can be either `signed` or `unsigned` and has an explicit size. **Signed** and **unsigned** refer to whether it’s possible for the number to be **negative** in other words, whether the number needs to have a sign with it (`signed`) or whether it will only ever be `positive` and can therefore be represented without a sign (`unsigned`). It’s like writing numbers on paper: when the sign matters, a number is shown with a `plus` sign or a `minus` sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using **two’s complement** representation.

> # TWO's Complement:
>
> **Definition:** Two's Complement is a way of representing both `positive` and `negative` integers in binary form.
>
> In this system, the most significant bit (leftmost bit) is used to represent the sign of the number.
>
> # Example:
>
> ## **Unsigned Integers:**
>
> In Rust, `u32` represents an `unsigned 32-bit` integer.
>
> Binary representation of `5` in u32:
>
> `00000000 00000000 00000000 00000101`
>
> ## **Signed Integers:**
>
> In Rust, `i32` represents a `signed 32-bit` integer.
>
> Binary representation of `-5` in `i32` using Two's Complement:
>
> Binary representation of 5:
>
> `00000000 00000000 00000000 00000101`
>
> Invert all bits (change `0s` to `1s` and vice versa):
>
> `11111111 11111111 11111111 11111010`
>
> Add `1` to the inverted value:
>
> `11111111 11111111 11111111 11111011`
>
> **So, `-5` in Two's Complement would be**
>
> `11111111 11111111 11111111 11111011`.
>
> # How it Works:
>
> - Positive numbers remain the same in both unsigned and signed representations.
> - To represent negative numbers, Two's Complement inverts all the bits of the positive number and adds 1 to the result.

Each **signed** variant can store numbers from

> $-(2^{n-1})\hspace{10pt}to\hspace{10pt}2^{n-1} - 1,\hspace{5pt}inclusive$

where `n` is the number of bits that variant uses.

So an `i8` can store numbers from $-(2^7)$ to $2^7 - 1$, which equals `-128` to `127`.

**Unsigned** variants can store numbers from

> $0\hspace{10pt}to\hspace{10pt}2^n - 1\hspace{5pt}$

so a `u8` can store numbers from $0$ to $2^8 - 1$, which equals `0` to `255`.

The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

# Integer Overflow

Let’s say you have a variable of type `u8` that can hold values between `0` and `255`. If you try to change the variable to a value outside that range, such as `256`, integer overflow will occur, which can result in one of two behaviors.

- When you’re compiling in `debug` mode, Rust includes checks for integer overflow that cause your program to `panic` at runtime if this behavior occurs. Rust uses the term `panicking` when a program exits with an error.
- When you’re compiling in release mode with the `--release` flag, Rust **does not** include checks for integer overflow that cause `panics`.

# Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.

Rust’s floating-point types are `f32` and `f64`, which are `32 bits` and `64 bits` in size, respectively. **The default type is `f64`** because on modern CPUs, it’s roughly the same speed as `f32` but is capable of more precision.

**All floating-point types are signed.**

Here’s an example that shows floating-point numbers in action:

Filename: src/main.rs

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

# Numeric Operations

Rust supports the basic mathematical operations you’d expect for all the number types: **addition, subtraction, multiplication, division, and remainder** (which are most used numeric operations).

```rust
fn main() {
// addition
let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

}
```

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable.

# Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible values: `true` and `false`. Booleans are `one byte` in size. The Boolean type in Rust is specified using `bool`. For example:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if` expression. We’ll cover how if expressions work in Rust in the **Control Flow** section.

# Character Type

Rust’s `char` type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.

Take a look at the following example:

```rust
fn print_type_of<T>(_: &T, var: &str) {
    println!("Type of {var} is {}", std::any::type_name::<T>())
}

fn main() {
    let c = 'z'; // char is always specified with single quotes, string literals are specified in double quotes
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let double_quotes = "s";

    print_type_of(&c, "c"); // char
    print_type_of(&z, "z"); // char
    print_type_of(&heart_eyed_cat, "heart_eyed_cat"); // char
    print_type_of(&double_quotes, "double_quotes"); // &str
}
```

Output:

```bash
cargo run
   Compiling data_types v0.1.0 (file:///projects/05_data_types)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/data_types`
Type of c is char
Type of z is char
Type of heart_eyed_cat is char
Type of double_quotes is &str
```

# Tuple Type

A `tuple` is a general way of grouping together a number of values with a variety of types into one compound type. **Tuples have a fixed length: once declared, they cannot grow or shrink in size.**

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

We’ve added optional type annotations in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to **destructure** a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}"); // 6.4
}
```

This program first creates a tuple and binds it to the variable `tup`. It then uses a pattern with let to take `tup` and turn it into three separate variables, `x`, `y`, and `z`. This is called **destructuring** because it breaks the single tuple into three parts (similar to javascript).

We can also access a tuple element directly by using a period `(.)` followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;
}
```

This program creates the tuple `x` and then accesses each element of the tuple using their respective `indices`. As with most programming languages, the first `index` in a tuple is `0`.

The tuple without any values has a special name, `unit`. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.

# Array Type

Another way to have a collection of multiple values is with an `array`. Unlike a `tuple`, **every element of an array must have the same type**. Unlike arrays in some other languages, arrays in Rust have a **fixed length**.

We write the values in an array as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the `stack` rather than the `heap` or when you want to ensure you always have a fixed number of elements.

An `array` isn’t as flexible as the `vector` type, though. A `vector` is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

If you’re unsure whether to use an `array` or a `vector`, chances are you should use a `vector`. Future topics discusses `vector`s in more detail.

However, arrays are more useful when you know the number of elements will not need to change.

For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

## Typing Rust Arrays

You write an array’s type using square brackets with the **\*type of each element, a semicolon,** \*and then **the number of elements in the array**, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of **each element**. After the semicolon, the number `5` indicates the **array contains five elements**.

## Initializing an array with similar values

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise way.

## Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be allocated on the **stack**. You can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1` because that is the value at index `[0]` in the array. The variable named `second` will get the value `2` from index `[1]` in the array.

## Invalid Array Element Access

Let’s see what happens if you try to access an element of an array that is past the end of the array.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[10])
}
```

After running `cargo run`, we get the following error:

```bash
   Compiling data_types v0.1.0 (file:///projects/05_data_types)
error: this operation will panic at runtime
  --> src/main.rs:97:20
   |
97 |     println!("{}", a[10])
   |                    ^^^^^ index out of bounds: the length is 5 but the index is 10
   |
   = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `data_types` (bin "data_types") due to previous error
```

The program resulted in a **runtime error** at the point of using an invalid value in the indexing operation.

The program exited with an error message and didn’t execute the final `println!` statement.

When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic.
