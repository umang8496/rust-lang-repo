
# The Rust Programming Language

---

## Understanding Ownership

Ownership in rust talks about how a Rust program manages its memory.  
Every programming language has a way or approach to manage the memory.  
Some languages have GC (garbage collector) that regularly looks for no-longer used memory as the program runs.  
Whereas, in other languages, the programmer must explicitly allocate and free the memory.  

Rust has a different mechanism for managing memory resources altogether.  
Here, the memory is managed through a system of ownership with a set of rules that the compiler checks.  
If any of the rules are violated, the program will not be compiled.  
None of the features of ownership will slow down the program while it's running.  

But before getting into the rules of Ownership, lets first understand how memory is used by a program in general.  

---

### The Stack and the Heap

Both the stack and the heap are parts of the memory available to our code to use at runtime, but they are structured in different ways.  
The stack stores data in LIFO (last in first out) manner, and is more organized than heap.  
All data stored on the stack must have a known, fixed size.  
Data with an unknown size at compile time or a size that might change must be stored on the heap instead.  

The heap is less organized, when we put data on the heap, we request a certain amount of space.
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.  
Because the pointer to the heap is a known, fixed size, we can store the pointer on the stack, but when we want the actual data, we must follow the pointer.  
Also, creating a piece of data on stack is easier and faster than that on the heap.  
Accessing data in the heap is slower than accessing data on the stack because we have to follow a pointer to get there.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap,  
and cleaning up unused data on the heap so we don't run out of space are all problems that ownership addresses.  

---

Ownership Rules
    - every value in Rust has an owner
    - there can be oly one owner for a value at a time
    - when the owner goes out of scope, the associated value value also goes out of the scope

When a variable goes out of scope, Rust calls a special function for us.  
This function is called drop, and it's where the author of String can put the code to return the memory.  
Rust calls drop automatically at the closing curly bracket.

Quite like the other language, the concept of scope remains the same in Rust as well.  
Here, a variable cannot be accessed outside of its scope.  

```rust
    fn main() {
        {
            let var = "rustlang";
            println!("var is {}", var);    // this is fine
        }
        println!("var is {}", var);    // var is out of scope here
    }

     |
     |     println!("var is {}", var);    // var is out of scope here
     |                           ^^^ not found in this scope
     |
```

Since the variables on stack is easy to copy, as the size and type of the variables is known at compile time, the ownership concept does not have visible effect there.  
But for the variables defined inside the heap memory, the copy or manipulation operations are quite complex and expensive.  
This is where the concept of ownership shines.  

---

### Ways Variables and Data Interact: (MOVE)

Multiple variables can interact with the same data in different ways in Rust.  
For example, the below written peice of code is absolutely fine and will run perfectly.

```rust
    let x = 5;
    let y = x;
    println!("x is {} and y is {}", x, y);
```

Both x and y are of scaler type and since their respective size are know at compile-time, they are defined on the stack.  
Hence, because of the `Copy` trait, its easier to perform the copy operation.  

But for the type of data stored on heap, such as String, things become a little complex.  

```rust
    // let s1 = "rustlang";    // the string literal declaration will work
    let s1 = String::from("rustlang");    // string value being defined on the heap
    let s2 = s1;
    println!("s1 is {} and s2 is {}", s1, s2);

    error[E0382]: borrow of moved value: `s1`
     |
     |     let s1 = String::from("rustlang");
     |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
     |     let s2 = s1;
     |              -- value moved here
     |     println!("s1 is {} and s2 is {}", s1, s2);
     |                                       ^^ value borrowed here after move
     |
```

Since the `String` type does not implement the `Copy` trait, so s1 and s2 cannot have access to the same value simultaneously.  
Here, we say that the value of the s1 has `moved` to s2.  

Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.  
This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.  
This is known as a double free error and is one of the memory safety bugs we mentioned previously.  
Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.  

To ensure memory safety, after the line `let s2 = s1`, Rust considers s1 as no longer valid.  
Therefore, Rust doesn't need to free anything when s1 goes out of scope.  

That solves our problem!  
With only s2 valid, when it goes out of scope, it alone will free the memory, and we are done.

In addition, there's a design choice that's implied by this: Rust will never automatically create `deep` copies of our data.  
Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.  

---

### Ways Variables and Data Interact: (CLONE)

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`.  

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
```

When we see a call to clone, we know that some arbitrary code is being executed and that code may be expensive.  
It's a visual indicator that something different is going on.

---

### Ownership & Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.  
Passing a variable to a function will move or copy, just as assignment does.

```rust
    fn main() {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still use x afterward
    }
    // Here, x goes out of scope, then s. 
    // But because s's value was moved, nothing special happens

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    }
    // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

```

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error.  
These static checks protect us from mistakes.

---

### Return Values & Scope

Returning values can also transfer ownership. For example,  

```rust
    fn main() {
        let s1 = gives_ownership();         // gives_ownership moves its return value into s1
        let s2 = String::from("hello");     // s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    }
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {        // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("yours"); // some_string comes into scope
        some_string                              // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
        a_string  // a_string is returned and moves out to the calling function
    }
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.  
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.  

While this works, taking ownership and then returning ownership with every function is a bit tedious.  
What if we want to let a function use a value but not take ownership ?  
It's quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.  

Though, Rust does let us return multiple values using a tuple.  

```rust
    fn main() {
        let s1 = String::from("rustlang");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
```

But this is too much ceremony and a lot of work for a concept that should be common.  
Luckily for us, Rust has a feature for using a value without transferring ownership, called `references`.

---

## References & Borrowing

The problem with the last piece of code for computing the length of the `String` is that we have to return the `String` to the calling function so that we can use it there after the call to `calculate_length` is done.  

There we only need to access the `String` value for performing certain other operations.
And for that, a mere reference (in general terms) of the `String` is also sufficient.  

Well, rust has something of that sort as well. It is also known as `Reference`.  
A reference is like a pointer in that it's an address we can follow to access the data stored at that address; That data is owned by some other variable.  
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.  

```rust
    fn main() {
        let s1 = String::from("rustlang");
        let len = calculate_length(&s1);    // this is how we pass the reference of a value into other context 
        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
```

The `&s1` syntax lets us create a reference that refers to the value of `s1` but does not own it.  
Because it does not own it, the value it points to will not be dropped when the reference stops being used.  
Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference.  

The scope in which the variable `s` is valid is the same as any function parameter's scope, but the value pointed to by the reference is not dropped when `s` stops being used because s doesn't have ownership.  
When functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.  
We call the action of creating a reference borrowing.  

```rust
    fn main() {
        let s = String::from("hello");
        change(&s);
    }
    fn change(some_string: &String) {
        some_string.push_str(", world");
    }

   error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    |
    | fn change(some_string: &String) {
    |                        ------- help: consider changing this to be a mutable reference: `&mut String`
    |     some_string.push_str(", world");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.  

---

### Mutable References

We can explicitly make the references as mutable.  

```rust
    fn main() {
        let mut s = String::from("hello");
        change(&mut s);
    }

    fn change(some_string: &mut String) {    // mutable reference
        some_string.push_str(", world");
    }
```

Mutable references have one big restriction: if we have a mutable reference to a value, we can have no other references to that value.  

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    error[E0499]: cannot borrow `s` as mutable more than once at a time
    |
    |     let r1 = &mut s;
    |              ------ first mutable borrow occurs here
    |     let r2 = &mut s;
    |              ^^^^^^ second mutable borrow occurs here
    |     println!("{}, {}", r1, r2);
    |                        -- first borrow later used here
```

This error says that this code is invalid because we cannot borrow s as mutable more than once at a time.  
The first mutable borrow is in r1 and must last until it's used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.  
The benefit of having this restriction is that Rust can prevent data races at compile time.  
A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There's no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when we are trying to track them down at runtime;  
Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones.

```rust
    let mut s = String::from("rustlang");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);

    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    |
    |     let r1 = &s; // no problem
    |              -- immutable borrow occurs here
    |     let r2 = &s; // no problem
    |     let r3 = &mut s; // BIG PROBLEM
    |              ^^^^^^ mutable borrow occurs here
    |     println!("{}, {}, and {}", r1, r2, r3);
    |                                -- immutable borrow later used here
```

We also cannot have a mutable reference while we have an immutable one to the same value.

Users of an immutable reference don't expect the value to suddenly change out from under them!  
However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used.  
For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

```rust
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
```

Even though borrowing errors may be frustrating at times, but remember that it'''s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime.

---

### Dangling References

In languages with pointers, it's easy to erroneously create a dangling pointer -- a pointer that references a location in memory that may have been given to someone else -- by freeing some memory while preserving a pointer to that memory.  
In Rust, by contrast, the compiler guarantees that references will never be dangling references:  
if we have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

```rust
    fn main() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String { // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String
        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
    
    // Danger!
```

Because `s` is created inside dangle, when the code of dangle is finished, `s` will be deallocated.  
But we tried to return a reference to it.  
That means this reference would be pointing to an invalid String. That's no good! Rust won't let us do this.

The solution here is to return the String directly:

```rust
    fn main() {
        let string = no_dangle();
    }

    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

---

### The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.  
- References must always be valid.

---

## The Slice Type

Slices let us reference a contiguous sequence of elements in a collection rather than the whole collection.  
A slice is a kind of reference, so it does not have ownership.  

The `first_word` function has a `&String` as a parameter. We don't want ownership, so this is fine. But what should we return ?  
We don't really have a way to talk about part of a string.  
However, we could return the index of the end of the word, indicated by a space.

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

We now have a way to find out the index of the end of the first word in the string, but there's a problem.  
We're returning a usize on its own, but it's only a meaningful number in the context of the `&String`.  
In other words, because it's a separate value from the String, there's no guarantee that it will still be valid in the future.  

The `string slice` type is represented by `&str`.  

```rust
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
```

Rather than a reference to the entire String, `hello` is a reference to a portion of the String, specified in the extra [0..5] bit.  
In the case of `let world = &s[6..11];`, `world` would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.  

Now lets rewrite the `first_word` function once again using the `&str` type.  

```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
```

Now when we call `first_word`, we get back a single value that is tied to the underlying data.  
The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

String Literals are String Slices

String Slices can be used as Parameters
If we have a string slice, we can pass that directly.  
If we have a String, we can pass a slice of the String or a reference to the String.  
This flexibility takes advantage of deref coercions.  

---

### Other Slice Types  

```rust
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
```

This slice has the type &[i32].  
It works the same way as string slices do, by storing a reference to the first element and a length.  

---
