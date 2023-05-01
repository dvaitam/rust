use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n : usize = line.trim().parse().expect("not int");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let a : Vec<i32> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(); 
        let mut m : HashSet<i32> = HashSet::new();
        m.insert(0);
        for i in 1..(1<<n) {
            let mut sm = 0;
            for j in 0..n {
                if i & (1<<j) != 0 {
                    sm += a[j]
                }
            }
            m.insert(sm);
        }
        if m.len() == (1<<n) {
            println!("NO");
        }else{
            println!("YES");
        }
    }
}
