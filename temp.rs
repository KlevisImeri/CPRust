use std::io::{stdin, stdout, BufWriter, Read, Write};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    // let mut scan = stdin().lines();
    let mut scan = input.split_ascii_whitespace();
    macro_rules! next { ()=>{scan.next().unwrap().parse().unwrap()}; }
    // macro_rules! next { ()=>{scan.next().unwrap().unwrap().parse().unwrap()}; }
    let mut out = BufWriter::new(stdout().lock());
    macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap() }; }
    // macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap(); out.flush().unwrap() }; }

    let mut solve = || {

    };

    let t: i64 = 1; for _ in 0..t { solve(); }
    // solve();
}
