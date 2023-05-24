use std::collections::HashMap;
use std::io;
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: u32 = line.trim().parse().expect("not a number");
    let mut cmap: HashMap<char, usize> = HashMap::new();
    for i in 0..26 {
        cmap.entry(ASCII_LOWER[i]).or_insert(i);
    }
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let s: Vec<char> = line.trim().chars().collect();
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for i in 0..s.len() {
            let count = char_map.entry(s[i]).or_insert(0);
            *count += 1;
        }
        let mut chars: Vec<&char> = char_map.keys().into_iter().collect();
        chars.sort();
        let mut first: Vec<char> = Vec::new();
        let mut second: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            if let Some(top) = first.pop() {
                if cmap[&chars[i]] == cmap[&top] + 1 || cmap[&chars[i]] + 1 == cmap[&top] {
                    second.push(*chars[i]);
                    first.push(top);
                } else {
                    first.push(top);
                    first.push(*chars[i]);
                }
            } else {
                first.push(*chars[i]);
            }
        }
        if let Some(stop) = second.pop() {
            if cmap[&stop] + 1 == cmap[&first[0]] || cmap[&stop] == cmap[&first[0]] + 1 {
                second.push(stop);
                if let Some(top) = first.pop() {
                    if cmap[&top] + 1 == cmap[&second[0]] || cmap[&top] == cmap[&second[0]] + 1 {
                        println!("No answer");
                        continue;
                    } else {
                        first.push(top);
                    }
                }
            } else {
                second.push(stop);
                for i in 0..second.len() {
                    for _j in 0..char_map[&second[i]] {
                        print!("{}", second[i]);
                    }
                }
                for i in 0..first.len() {
                    for _j in 0..char_map[&first[i]] {
                        print!("{}", first[i]);
                    }
                }

                println!();
                continue;
            }
        }
        for i in 0..first.len() {
            for _j in 0..char_map[&first[i]] {
                print!("{}", first[i]);
            }
        }
        for i in 0..second.len() {
            for _j in 0..char_map[&second[i]] {
                print!("{}", second[i]);
            }
        }
        println!();
    }
}
