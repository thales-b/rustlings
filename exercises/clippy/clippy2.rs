// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    if let Some(12) = Some(12) {
        res += 12;
    }
    println!("{}", res);
}
