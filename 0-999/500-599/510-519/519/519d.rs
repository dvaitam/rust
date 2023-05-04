use std::io;
use std::collections::HashMap;
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
fn main() {
   let mut char_map : HashMap<char, i64> = HashMap::new();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let a : Vec<i64> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    for i in 0..26{
        char_map.entry(ASCII_LOWER[i]).or_insert(a[i]);
    }
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let s : Vec<char> = line.trim().chars().collect();
    let mut ans : i64 = 0;
    for i in 0..26 {
        let mut m : HashMap<i64, i64> = HashMap::new();
        let mut sm : i64 = 0;
        for j in 0..s.len() {
            sm += char_map[&s[j]];
            if s[j] == ASCII_LOWER[i]{
                if m.contains_key(&(sm-a[i])) {
                    ans += m[&(sm-a[i])];
                }
                let count = m.entry(sm).or_insert(0);
                *count += 1;
            }
         }
    }
    println!("{}",ans);
   
}
