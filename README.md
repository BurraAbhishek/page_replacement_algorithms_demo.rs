# page_replacement_algorithms_demo.rs
A demonstration of page replacement algorithms using Rust, given the page reference string

This program currently supports 3 page replacement algorithms:
- **First-In-First-Out**: Whichever page goes in first is replaced first.
- **Least Recently Used**: Whichever page was least recently used is replaced first.
- **Optimal**: Whichever page is not likely to be used in the future is replaced first.

## Getting started
1. You will need a Rust compiler to run these programs. To install Rust, go to https://www.rust-lang.org/ and follow the instructions given in the page.
2. Rust uses a package manager known as Cargo (and each package is called a crate). Cargo generally comes pre-installed with Rust.
3. Open your Terminal or Command Prompt, and type `cargo`. You should be able to see the text `Rust's packgae manager` along with the list of options.
4. Either clone this repository using `git clone https://github.com/BurraAbhishek/page_replacement_algorithms_demo.rs.git` (requires a [Git](https://git-scm.com/) installation), or download it.
5. Open the folder. You should be able to see the license, README, src directory (contains the source code) and Cargo.toml (Package dependency manager), and some other files. Open your terminal or command prompt in this folder.
6. To run this program, type the command `cargo run` and follow the instructions given in the program.
