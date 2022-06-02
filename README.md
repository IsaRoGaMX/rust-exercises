# rust-exercises
 A repository with rust exercises.

 ## Sources
  - [👌 Curso de RUST desde CERO by Luis Serrano Donaire](https://www.youtube.com/playlist?list=PLAMfQH2NKM_tyKzBV1iJf5L8j7oJl6KHl)
  - [Codewars Katas](https://www.codewars.com)
  - [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)

## Install Rust
Install on macOS, Linux, or another Unix-like OS.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


## Cargo
- Create rust project on new folder

`cargo new <project-folder>`

- Create rust project on existing folder

`cargo init`

- Run project

`cargo run`

## Data Types

Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### Integer Types

| Length | Signed | Unsigned |
| :---: | :---: | :---:
| 8 bit | i8 | u8
| 16 bit | i16 | u16
| 32 bit | i32 | u32
| 64 bit | i64 | u64
| 128 bit | i128 | u128
| arch | isize | usize

> The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

### Integer Literals in Rust

| Number literals | Example |
| :---: | :---: |
| Decial | `98_222` |
| Hex | `0xff` |
| Octalt | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8`only) | `b'A'` |

---

## Notes
- Rust use snake case
- In Rust variables are inmutables by default. To declare mutable variable use `mut` keyword. Example: `let mut variable = 5;`
- To define constants use uppercase. Example: `const MY_CONSTANT: i32 = 9999;`

- asd

