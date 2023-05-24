
# The Rust Programming Language

---

## What is Rust ?

Rust is a high-performance, statically-typed multi-paradigm programming language.  
With the main focus on safety and performance.  
This language helps developers create robust and secure applications.  
Compared to C/C++, which struggles with memory errors and developing concurrent programs, Rust has already solved these problems.  
The open source Rust community describes the language as fast, reliable and productive.  

Rust code was originally developed by software developer Graydon Hoare while working at Mozilla Research in 2006
And since 2021, it has been maintained by the Rust Foundation.  

The top benefit of Rust programming language is its adept memory management.  
Rust handles the concept differently in that it doesn’t use a garbage collector as other programming languages do.  
Instead, Rust uses a borrow checker to track variable scope and object lifetime while simultaneously administering high-quality memory safety and stopping concurrent data races.  

The benefits of Rust programming don’t stop at memory management.  
It’s fast and reliable for creating web apps and creating cross-platform applications, and it can integrate with pre-existing code.

One of the other major benefits of Rust programming language is that it is well-suited for projects that demand extremely high performance.  
Its ability to process large amounts of data and CPU-intensive operations makes it a strong competitor in the developer space.

### Explore the following resources for learning the Rust programming language  

- [Rust-Lang Stable Book](https://doc.rust-lang.org/book/)  
- [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/)  
- [Rust API Documentation](https://doc.rust-lang.org/std/)
- [Comprehensive Rust By Google](https://google.github.io/comprehensive-rust/welcome.html)
- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)  
- [Exercism](https://exercism.org/dashboard)  
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Rust Language Cheat Sheet](https://cheats.rs/)

### Read the following blogs/articles to know about the Rust Programming Language  

- [Rust programming language by Antony Saba](https://about.gitlab.com/blog/2020/07/21/rust-programming-language/)  
- [What is Rust programming language by Iryna Deremuk](https://litslink.com/blog/what-is-rust-programming-language)  
- [What is rust and why is it so popular](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)  
- [Rust Guide by Gints Dreimanis](https://serokell.io/blog/rust-guide)  


### Rust Cheat Sheets

- [Quick Ref](https://quickref.me/rust.html)
- [github.com/donbright](https://github.com/donbright/rust-lang-cheat-sheet)

---  

## What is Cargo ?

Cargo is a build system and package manager for Rust.  
It helps developers download and manage dependencies and assists in creating Rust packages.  
Packages in Rust are often called "crates" in the Rust community.

A program often depends on external libraries or dependencies to run, which enables us to write applications that perform tasks that   
we don't know how to code or we don't want to spend time coding.  
All our dependencies will be listed in this file.  
At this point, we do not have any dependencies for our new program.  

Open the Cargo.toml file and view its contents:

``` toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["user <user@mail.com>"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
```

### Read the following blogs/articles to know about the Cargo package manager  

- [Getting started with the Rust package manager, Cargo](https://opensource.com/article/20/3/rust-cargo)
- [Learning Cargo](https://binx.io/2018/11/27/learning-cargo/)

---  

## CLI for interacting with Rust and Cargo  

``` rust
$ rustc --version  
describes the currently installed version of rust
```

``` rust
$ rustup update
updates the currently installed version of rust
```

``` rust
$ rustup self uninstall  
uninstalls the currently installed version of rust
```

``` rust
$ rustup doc  
open the local copy of the documentation in the browser
```

``` rust
$ rustc main.rs  
compiles the main.rs file
```

``` rust
$ cargo --version
describes the currently installed version of cargo
```

``` rust
$ cargo new hello_cargo
creates a binary for rust project (follows a template)
```

``` rust
$ cargo new --lib hello_cargo
creates a library for rust project (follows a template)
```

``` rust
$ cargo run  
compiles the code and then runs the resulting executable all in one command
```

``` rust
$ cargo check  
quickly checks the code to make sure it compiles but does not produce an executable
```

``` rust
$ cargo build (--release)
builds the executable file out of the project source code
```

``` rust
$ cargo clean
removes all intermediate files created by the earlier build process
```

``` rust
$ cargo test
executes the the unit-tests and integration-tests 
```

---  
