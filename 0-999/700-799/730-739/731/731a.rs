use std::io;
use std::collections::HashMap;
use std::cmp::min;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
fn main() {
   
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    if s.ends_with('\n') {
        s.pop();
       // println!("popped newline");
    }
    if  s.ends_with('\r'){
        s.pop();
        //println!("popped tab");

    }
    if s.ends_with('\n') {
        s.pop();
      //  println!("popped newline");
    }
    if  s.ends_with('\r'){
        s.pop();
        //println!("popped tab");

    }
    let s : Vec<char> = s.chars().collect();
    let mut m : HashMap<char, i32> = HashMap::new();
    for i in 0..26{
        m.entry(ASCII_LOWER[i]).or_insert(i as i32);
    }
    let first : i32  = 0;
    let mut ans : i32  = 0;
    for i in 0..s.len() {
        if !m.contains_key(&s[i]) {
            continue;
        }
        if i == 0 {
            ans += min(m[&s[i]]-first, (m[&s[i]]-(first+26)).abs());
        }else{
            ans += min(min((m[&s[i]]-m[&s[i-1]]).abs(), (m[&s[i]]+26-m[&s[i-1]]).abs()), (m[&s[i-1]]+26-m[&s[i]]).abs());
        }
    }
    println!("{}", ans);
    
}
