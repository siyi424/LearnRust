# Notes for Beginner
Book: The Rust Programming Language
Rust is not an OOP language. It doesn't have class and inherite concepts, but can use enum and structs to behave like an OOP language.
Rust is not a pure functional programming language either but it does support functional programming concepts such as closures and iterators.

## Chapter 1 Getting Started
- how to install rust
- how to write a helloWorld
- How to ust Cargo, Rust's package manager, build tool

```
- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
```

1. 文件**命名**：underscore命名法 eg. hello_word.
2. .rs文件写好后，输入"rustc xx.rs"命令进行**编译**，在相同目录下生成同名的编译文件。再输入"./main"打开文件直接运行。
```shell
rustc xx.rs
./main
```
3. **Automatic Formatting with rustfmt**:
```shell
rustup component add rustfmt  # install rsutfmt
cargo fmt  # format all rust files
rustfmt xx.rs  # format the specific file
```
4. rust println！的细节：前面4个空格；结尾分号；
println！, calls a rsut macro. 
println, a functional expression.
```rust
    println!("hello,world!");
```
5. Cargo is Rust's build system and package manager.For real projects, we prefer using Cargo.
```shell
cargo new hello_cargo
```
创建一个新项目hello_cargo。如果已有了一个git仓库，就不重新生成了。否则，也会自动生成一个git repo。

In Cargo.toml file, the init info listed like:
```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
- [pakage]:  indicates that the following statements are configuring a package. This hello_cargo project is a package.
- [dependencies]: to list dependencies. 

6. **In Rust, packages of code are referred to as crates.**
7. Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.
8. **building and running**:
```shell
cargo build
./target/debug/hello_cargo
```
1. creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows)
2. Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project. 

```shell
cargo run
```
compile and run

```shell
cargo check
```
This command quickly checks your code to make sure it compiles but doesn’t produce an executable.We can build a project without producing a binary to check for errors using cargo check.
- When a project is too big, the compile time is much longer, you can use "cargo check" to know if your project is still compiling!
- also, use it to make sure it compiles

```shell
cargo build --release
```
We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.






## Chapter 2 Programming a Guessing Game

## chapter 3 Common Programming Concepts
- Variables and Mutability
- Datatypes
- functions
- comments
- control flow

## chapter 4 understanding ownership
- ownership
- reference and borrowing
- the slice type

## chapter 5 structs
- structs
- methods

## chapter6 enums and pattern matching
- enums
- match
- if let

## chapter 7 packages, crates and modules
- module system
- privacy rules
- API

## chapter 8 common collections
- vectors
- strings
- hashmaps

## chapter 9 error handling

## chapter 10 generic types, traits and lifetimes
- generics
- traits
- lifetimes

## chapter 11 writing automated tests

## chapter 12 an IO project

## chapter 13 functional language features
- closures 
- iterators

## chapter 14 Cargo and Crates.io

## chapter 15 smart pointers

## chapter 16 fearless concurrency
 
## chapter 17 OOP features

## chapter 18 patterns and matching

## chapter 19 advanced features
- unsafe Rust
- macros
- others

## chapter 20 final projects: multithreaded web server
