use std::io;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not int");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let n: usize = line.trim().parse().expect("not int");
        let (mut first1, mut first2) = (2*n-1, 1);
        let mut i = 0;
		while i < n {
            print!("{} {} ", first1, first2);
			first1 -= 2;
			first2 += 2;
            i += 2;
		}
        println!();
		(first1, first2) = (2, n+2);
        i = 0;
		while i < n {
            print!("{} {} ", first1, first2);
			first1 += 2;
			first2 += 2;
            i += 2;
		}
        println!();
    }
    
}
