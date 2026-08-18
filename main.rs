use std::{cmp::*, collections::*, io::*, mem::*,};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    // let mut scan = stdin().lines();
    let mut scan = input.split_ascii_whitespace();
    macro_rules! next { ()=>{scan.next().unwrap()}; }
    // macro_rules! next { ()=>{scan.next().unwrap().unwrap()}; }
    macro_rules! nextp { ($t:ty)=>{next!().parse::<$t>().unwrap()}; }
    macro_rules! nextstr { ()=>{next!().as_bytes().to_vec()}; }
    let mut out = BufWriter::new(stdout().lock());
    macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap() }; }
    // macro_rules! put { ($($arg:tt)*) => { write!(out, $($arg)*).unwrap(); out.flush().unwrap() }; }
    macro_rules! dbg { ($($arg:tt)*) => {}; }

    let t = nextp!(usize); dbg!(t);
    let mut solve = || {
        let n = nextp!(usize); dbg!(n);
        
        let mut r = vec![0usize; n];
        let mut degin = vec![0usize; n];
        for i in 0..n {
            r[i] = nextp!(usize) - 1;
            degin[r[i]]+=1;
        }
        dbg!(&r, &degin);

        let mut b = vec![];
        for i in 0..n {
            if degin[i]==0 {
                b.push(i);
            }
        }

        let mut ans = vec![1usize; n];
        let mut mx = 0;
        while let Some(x) = b.pop() {
            mx = mx.max(ans[x]);
            let next = r[x];
            ans[next]+=ans[x];
            degin[next]-=1;
            if degin[next]==0 {
                b.push(next);
            }
        }
        dbg!(&ans);

        put!("{}\n", mx+2);
    };

    for _ in 0..t { solve(); }
}


