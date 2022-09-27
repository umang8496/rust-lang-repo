
# The Rust Programming Language

---

## Variables & Mutability  

In Rust, be default, variables are immutable.  
This is one of many nudges Rust gives us to write our code in a way that takes advantage of the safety and easy concurrency that Rust offers.  
When a variable is immutable, once a value is bound to a name, you can’t change that value.  

```rust
    let number = 10;
    println!("number is {}", number);
    number = 20;
    println!("number is {}", number);

    |     let number = 10;
    |         ------
    |         |
    |         first assignment to `number`
    |         help: consider making this binding mutable: `mut number`
    |     println!("number is {number}");
    |     number = 20;
    |     ^^^^^^^^^^^ cannot assign twice to immutable variable
```

It's important that we get compile-time errors when we attempt to change a value that's designated as immutable because this very situation can lead to bugs.  
If one part of our code operates on the assumption that a value will never change and another part of our code changes that value,  
it’s possible that the first part of the code won’t do what it was designed to do.  

But mutability is required to perform certain operations during the runtime.  
Rust provides us with a `mut` keyword which declares a variable to be mutable.  

```rust
    let mut number = 10;
    println!("number is {}", number);
    number = 20;
    println!("number is {}", number);

    number is 10
    number is 20
```

Like immutable variables, **Constants** are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.  
First, we cannot use the `mut` keyword with constants. Constants aren’t just immutable by default—they’re always immutable.  
Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.  
The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.  

```rust
    #![allow(unused)]
    fn main() {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    }
```

Rust’s naming convention for constants is to use all uppercase with underscores between words.  
Constants are valid for the entire time a program runs, within the scope they were declared in.  

We can declare a new variable with the same name as a previous variable.  
**Rustaceans** say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when we use the name of the variable.  
In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.  
We can shadow a variable by using the same variable's name and repeating the use of the let keyword as follows:

```rust
    fn main() {
        let x = 10;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
        println!("The value of x is: {x}");
    }

    The value of x in the inner scope is: 22
    The value of x is: 11
```

**Shadowing** is different from marking a variable as `mut`, because we will get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.  
By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between **mut** and **shadowing** is that because we are effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.  

```rust
    let spaces = "    ";
    let spaces = spaces.len();
```

The first `spaces` variable is a string type and the second `spaces` variable is a number type.  
Shadowing spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.  
However, if we try to use `mut` for this, as shown here, we will get a compile-time error.

```rust
    let mut spaces = "    ";
    spaces = spaces.len();

    |     let mut spaces = "    ";
    |                      ------ expected due to this value
    |     spaces = spaces.len();
    |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

---

## Data Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.  
Rust is a statically typed language, which means that it must know the types of all variables at compile time.  
The compiler can usually infer what type we want to use based on the value and how we use it.  
When the types are not possible to be inferred, then we must use annotations to help Rust compiler.  

Rust has two categories for data types: **Scaler Types** & **Compound Types**  

### Scaler Types

A scalar type represents a single value.  
Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.  

#### Integer Types

| Length  | Signed | Unsigned |
|:-------:|:------:|:--------:|
| 8-bit   |  i8    |   u8     |
| 16-bit  |  i16   |   u16    |
| 32-bit  |  i32   |   u32    |
| 64-bit  |  i64   |   u64    |
| 128-bit |  i128  |   u128   |
| arch    |  isize |   usize  |

Signed numbers are stored using 2's complement representation.  
Each signed variant can store numbers from -2^(n-1) to 2^(n-1)-1 inclusive, where n is the number of bits that variant uses.  
Unsigned variants can store numbers from 0 to 2^(n-1).  

Additionally, the isize and usize types depend on the architecture of the computer program is running on.  
i32 is the default type for integers.  

#### Floating-point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.  
Rust's floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.  
The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.  
All floating-point types are signed.  
Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.  

#### Numeric Operations

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
        let floored = 2 / 3; // Results in 0

        // remainder
        let remainder = 43 % 5;
    }
```

#### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible values: true and false.  
Booleans are one byte in size. The Boolean type in Rust is specified using bool.  

#### The Character Type

Rust’s char type is the language’s most primitive alphabetic type.  

```rust
    fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
```

Rust's char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.  
Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.  
Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.  

### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.  

#### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.  
Tuples have a fixed length: once declared, they cannot grow or shrink in size.  

We create a tuple by writing a comma-separated list of values inside parentheses.  
Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same.

```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        let (x, y, z) = tup;
    }
```

The tuple without any values has a special name, unit.  
This value and its corresponding type are both written () and represent an empty value or an empty return type.  
Expressions implicitly return the unit value if they don’t return any other value.  

#### The Array Type

Another way to have a collection of multiple values is with an array.  
Unlike a tuple, every element of an array must have the same type.  
Unlike arrays in some other languages, arrays in Rust have a fixed length.  

```rust
    fn main() {
        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    }
```

Arrays are useful when we want our data allocated on the stack rather than the heap.  
An array isn't as flexible as the vector type, though.  
A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.  
If we are unsure whether to use an array or a vector, chances are we should use a vector.
However, arrays are more useful when you know the number of elements will not need to change.  

```rust
    #![allow(unused)]
    fn main() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let a = [3; 5];
        //  This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    }
```

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. We can access elements of an array using indexing.  

```rust
    fn main() {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        let sixth = a[5];    // runtime error
    }
```

The program resulted in a runtime error at the point of using an invalid value in the indexing operation.  
When we attempt to access an element using indexing, Rust will check that the index we have specified is less than the array length.  
If the index is greater than or equal to the length, Rust will panic.  
This check has to happen at runtime, especially in this case, because the compiler can't possibly know what value a user will enter when they run the code later.

This is an example of Rust's memory safety principles in action.  
In many low-level languages, this kind of check is not done, and when we provide an incorrect index, invalid memory can be accessed.  
Rust protects us against this kind of error by immediately exiting instead of allowing the memory access and continuing.  

---

## Functions

Functions are prevalent in Rust code.  
Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.  

```rust
    fn main() {
        println!("Hello, world!");

        another_function();
    }

    fn another_function() {
        println!("Another function.");
    }
```

We can define functions to have parameters, which are special variables that are part of a function's signature.  
When a function has parameters, we can provide it with concrete values for those parameters.  
Technically, the concrete values are called arguments, and arguments are the variables in a function's definition or the concrete values passed in when we call a function.  

```rust
    fn main() {
        print_labeled_measurement(5, 'h');
    }

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }
```

In function signatures, we must declare the type of each parameter.  
This is a deliberate decision in Rust's design: requiring type annotations in function definitions means the compiler almost never needs us to use them elsewhere in the code to figure out what type we mean.  
The compiler is also able to give more helpful error messages if it knows what types the function expects.

### Statements and Expressions

Since Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions.  
Statements are instructions that perform some action and do not return a value.  
Expressions evaluate to a resulting value.  

```rust
    fn main() {
        // its a statement
        let a = 6;

        // its an error
        let x = (let y = 6);

        // its an expression
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }
```

### Functions with Return Values

Functions can return values to the code that calls them.  
We don't name return values, but we must declare their type after an arrow (->).  
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.  
We can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

```rust
    fn five() -> i32 {
        5
    }

    fn main() {
        let x = five();

        println!("The value of x is: {x}");
    }
```

Lets look at another example:  

```rust
    fn main() {
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
```

But if we change the expression `x + 1` into a statement, then, see what happens:

```rust
    fn main() {
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1;
    }
```

---

## Control Flow

The ability to run some code depending on if a condition is true, or run some code repeatedly while a condition is true, are basic building blocks in most programming languages.  

### if Expressions

It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.

```rust
    fn main() {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
```

### Handling Multiple Conditions with else if

```rust
    fn main() {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }
```

Using too many `else if` expressions can clutter the code, so if we have more than one, we might want to refactor our code.  
Rust provides a powerful branching construct called `match` for these cases.

### Using if in a let Statement

Since `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.

```rust
    fn main() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {number}");
    }
```

Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.

```rust
    fn main() {
        let condition = true;
        let number = if condition { 5 } else { "six" };
        println!("The value of number is: {number}");
    }

    |
    |     let number = if condition { 5 } else { "six" };
    |                                 -          ^^^^^ expected integer, found `&str`
    |                                 |
    |                                 expected because of this
```

### Repeating Code with loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until we explicitly tell it to stop.  
Fortunately, Rust also provides a way to break out of a loop using code.  
We can place the `break` keyword within the loop to tell the program when to stop executing the loop.

```rust
    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }
```

### Loop Labels to Disambiguate Between Multiple Loops

If we have loops within loops, `break` and `continue` apply to the innermost loop at that point.  
We can optionally specify a loop label on a loop that we can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.  
Loop labels must begin with a single quote.  

```rust
    fn main() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
    }
```

### Conditional Loops with while

A program will often need to evaluate a condition within a loop.  
While the condition is true, the loop runs.  
When the condition ceases to be true, the program calls `break`, stopping the loop.  
It's possible to implement behavior like this using a combination of `loop`, `if`, `else`, and `break`.  
However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop.  

```rust
    fn main() {
        let mut number = 3;
        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }
```

### Looping Through a Collection with for

We can choose to use the while construct to loop over the elements of a collection, such as an array.  

```rust
    fn main() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }
    
    the value is: 10
    the value is: 20
    the value is: 30
    the value is: 40
    the value is: 50
```

As a more concise alternative, we can use a for loop and execute some code for each item in a collection.  

```rust
    fn main() {
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}");
        }
    }

    the value is: 10
    the value is: 20
    the value is: 30
    the value is: 40
    the value is: 50
```

```rust
    fn main() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }

    3!
    2!
    1!
    LIFTOFF!!!
```

---
