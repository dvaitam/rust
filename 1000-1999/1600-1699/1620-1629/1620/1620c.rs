use std::io;
#[derive(Copy, Clone, PartialEq)]
struct Symbol {
    ch: char,
    count: usize,
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: u32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (n, k, x): (usize, usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let s: Vec<char> = line.trim().chars().collect();
        let mut symbols: Vec<Symbol> = Vec::new();
        for i in 0..n {
            if symbols.len() == 0 {
                symbols.push(Symbol { ch: s[i], count: 1 });
            } else {
                let last = symbols[symbols.len() - 1];
                if last.ch == s[i] {
                    let ll = symbols.len();
                    symbols[ll - 1].count += 1;
                //last.count++
                } else {
                    symbols.push(Symbol { ch: s[i], count: 1 });
                }
            }
        }
        for i in 0..symbols.len() {
            if symbols[i].ch == '*' {
                symbols[i].ch = 'b';
                symbols[i].count = symbols[i].count * k + 1;
            }
        }
        let mut bases: Vec<usize> = vec![1; 1];
        for i in (0..symbols.len()).rev() {
            if symbols[i].ch == 'b' {
                bases.push(symbols[i].count);
            }
        }
        let mut powers: Vec<usize> = vec![1; 1];
        for i in 1..bases.len() {
            powers.push(powers[i - 1] * bases[i]);
            if x <= powers[i] {
                break;
            }
        }
        let mut vals: Vec<usize> = vec![0; powers.len()];
        let mut tx = x;
        for i in (0..powers.len()).rev() {
            vals[i] = tx / powers[i];
            tx = tx % powers[i];
        }
        for i in 0..vals.len() {
            if vals[i] > 0 {
                vals[i] -= 1;
                break;
            } else {
                vals[i] = bases[i + 1] - 1;
            }
        }
        let mut ans: Vec<usize> = vec![0; symbols.len()];
        let mut j = 0;
        for i in (0..symbols.len()).rev() {
            if symbols[i].ch == 'b' {
                ans[i] = vals[j];
                j += 1;
                if j == vals.len() {
                    break;
                }
            }
        }
        for i in 0..symbols.len() {
            if symbols[i].ch == 'a' {
                for _j in 0..symbols[i].count {
                    print!("a");
                }
            } else {
                for _j in 0..ans[i] {
                    print!("b");
                }
            }
        }
        println!();
    }
}
