use std::{cmp::*, collections::*, io::*, mem::*,};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    // let mut scan = stdin().lines();
    let mut scan = input.split_ascii_whitespace();
    macro_rules! next { ()=>{scan.next().unwrap()}; }
    // macro_rules! next { ()=>{scan.next().unwrap().unwrap()}; }
    macro_rules! nextp { ()=>{next!().parse().unwrap()}; }
    macro_rules! nextstr { ()=>{next!().as_bytes().to_vec()}; }
    let mut out = BufWriter::new(stdout().lock());
    macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap() }; }
    // macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap(); out.flush().unwrap() }; }
    macro_rules! dbg { ($($arg:tt)*) => {}; }

    let t: i64 = nextp!(); dbg!(t);
    let mut solve = || {

    };

    for _ in 0..t { solve(); }
}
