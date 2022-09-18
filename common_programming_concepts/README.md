
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
