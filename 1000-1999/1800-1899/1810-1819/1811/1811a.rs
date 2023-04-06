use std::io;
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
        let (_dummy, d) = (
            split.next().unwrap(),
            split.next().unwrap(),
        );
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line");
            if s.ends_with('\n') {
                s.pop();
            }
        let mut ans = s.clone()+d;
        for i in 0..s.len() {
            if d[0..1] > s[i..i+1] {
                ans = [&s[..i], d , &s[i..]].concat();
                break;
            }
        }
        println!("{}", ans)
    }
}
