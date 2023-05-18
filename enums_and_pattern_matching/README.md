
# Enums and Pattern Maching

Where structs give us a way of grouping together related fields and data, like a `Rectangle` with its `width` and `height`,  
Enums give us a way of saying a value is one of a possible set of values.  
For example, we may want to say that `Rectangle` is one of a set of possible `Shapes` that also includes `Circle` and `Triangle`.  
To do this, Rust allows us to encode these possibilities as an enum.  

```rust
    enum IpAddrKind {
        V4,
        V6,
    }
```

We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

We can also define a fucntion which takes in an enum, like below:  

```rust
    fn route(ip_kind: IpAddrKind) {}
```

And this function can be called with any of the variants.  

```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```

Other variants of the same enum type:  

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // or

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
```

---

## The `Option` Enum  

The `Option` enum in Rust is a type that can either contain a value or not contain a value.  
It is used to represent the possibility of an outcome, such as whether a function call succeeds or fails.  

The Option enum has two variants:  

- `None`: This variant indicates that the Option does not contain a value.
- `Some(value)`: This variant indicates that the Option contains a value, which is stored in the value field.

The `Option` enum can be used to handle errors gracefully.  
For example, if a function call can fail, the function can return an `Option<T>`, where `T` is the type of the value that the function is supposed to return.  
The caller of the function can then check the `Option` to see if it contains a value.  
If the `Option` contains a value, the caller can use the value.  
If the `Option` does not contain a value, the caller can handle the error appropriately.  

The `Option` enum can also be used to represent the possibility of a value existing.  
For example, a function that searches for a value in a list can return an `Option<T>`, where `T` is the type of the value that the function is searching for.  
If the value is found, the `Option` will contain the value.  
If the value is not found, the `Option` will not contain a value.

The `Option` enum is a powerful tool that can be **used to handle errors** and represent the possibility of a value existing.  
It is a fundamental part of the Rust language and is used in many different parts of the standard library.

```rust
    enum Option<T> {
        None,
        Some(T),
    }
```

For example, lets see the following two examples:  

```rust
    fn find_value(list: &[i32], value: i32) -> Option<i32> {
        for i in 0..list.len() {
            if list[i] == value {
            return Some(list[i]);
            }
        }
        None
    }

    fn main() {
        let list = [1, 2, 3, 4, 5];
        let value = 3;
        let result = find_value(&list, value);
        if let Some(found_value) = result {
            println!("Found value: {}", found_value);
        } else {
            println!("Value not found");
        }
    }
```

```rust
    fn read_file(filename: &str) -> Option<String> {
        let mut file = File::open(filename).expect("Could not open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");
        Some(contents)
    }

    fn main() {
        let filename = "my_file.txt";
        let result = read_file(filename);
        if let Some(contents) = result {
            println!("Contents of file: {}", contents);
        } else {
            println!("Could not read file");
        }
    }
```

Now lets see how can we fetch or extract the value out of the `Option` enum or `Some(T)` variant.  

```rust
    fn main() {
        // Create an Option with a value
        let value = Some(1);

        // Use unwrap() to get the value
        let unwrapped_value = value.unwrap();
        println!("Unwrapped value: {}", unwrapped_value);

        // Create an Option without a value
        let value = None;

        // Use unwrap() to get the value
        let unwrapped_value = value.unwrap();
        // This will panic!

        // Use if let pattern to get the value
        if let Some(unwrapped_value) = value {
            println!("Unwrapped value: {}", unwrapped_value);
        }

        // Use match statement to get the value
        match value {
            Some(unwrapped_value) => println!("Unwrapped value: {}", unwrapped_value),
            None => println!("No value"),
        }
    }
```

---

## The match Control Flow Construct

The `match` control flow in Rust is a powerful way to handle different cases of a value.  
It is similar to the switch statement in other languages.  

The `match` statement takes a value and then matches it against different patterns.  
If the value matches a pattern, the code associated with the pattern is executed.  
If the value does not match any of the patterns, the `match` statement will fall through to the next statement.  

Here is an example of how the match statement can be used:

```rust
    fn main() {
        let value = 5;
        match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Other"),
        }
    }
```

The `match` statement can also be used to handle errors.  
For example, the following code will print the error message if the value of the variable `value` is not a number:  

```rust
    fn main() {
        let value = "hello";
        match value.parse::<i32>() {
            Ok(number) => println!("The number is {}", number),
            Err(error) => println!("Error: {}", error),
        }
    }
```

Here `Ok(T)` and `Err(E)` are part of `Result` enum which itself is a part of standard library.  

```rust
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.  
Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have `null`, thus making the billion-dollar mistake discussed earlier impossible.  

---

## Concise Control Flow with `if let`

`if let` is a control flow construct in Rust that allows us to check if a value matches a pattern and then execute some code if it does.  
It was introduced in Rust 1.25 to make it easier to handle errors and to write more concise code.  

```rust
    // Checking if a value is present
    let value = Some(1);
    if let Some(number) = value {
        println!("The number is {}", number);
    }
```

```rust
    // Converting a value to a different type
    let value = "123";
    if let Some(number) = value.parse::<i32>() {
        println!("The number is {}", number);
    }
```

```rust
    // Filtering a list
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|number| number % 2 == 0).collect();
    println!("{:?}", even_numbers);
```

`if let` solves the problem of handling errors and writing concise code in Rust.  
It is a more concise and readable way to handle errors than using the `match` statement.  
It also allows us to write more concise code when converting values to different types or filtering lists.  

```rust
    fn main() {
        let value = "hello";
        if let Some(number) = value.parse::<i32>() {
            println!("The number is {}", number);
        } else {
            println!("The value is not a number");
        }
    }
```

The above written code is more concise than the following code, which uses the `match` statement.  

```rust
    let value = "123";
    match value.parse::<i32>() {
        Ok(number) => println!("The number is {}", number),
        Err(error) => println!("Error: {}", error),
    }
```

---
