// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 0;
    let option = Some(12);
    if let Some(val) = option {
        res += val;
    } else {
        res += 1;
    }
    println!("{}", res);
}
