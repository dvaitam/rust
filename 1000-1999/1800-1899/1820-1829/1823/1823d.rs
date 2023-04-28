use std::io;
static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];
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
        let mut split = line.trim().split(" ");
    
        let (n, k): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let x : Vec<usize> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let c : Vec<usize> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut s : Vec<char> = vec!['a';n];
        let mut chars : Vec<char> = vec!['a';3];
		chars[0] = 'a';
		chars[1] = 'b';
		chars[2] = 'c';
		let mut possible = true;
		let mut curr = 0;
		for i in 0..k {
			if i == 0 {
				if c[i] > x[i] {
					possible = false;
					break;
				}
				for j in  0..(c[i]-3){
					s[j] = ASCII_LOWER[i+3];
				}
				for j in  (c[i] - 3)..c[i]{
					s[j] = chars[curr];
					curr += 1;
					curr %= 3;
				}
				for j in c[i]..x[i] {
					s[j] = chars[curr];
					curr += 1;
					curr %= 3;
				}
			} else {
				if c[i]-c[i-1] > x[i]-x[i-1] {
					possible = false;
					break;
				}
				for j in x[i-1]..(x[i-1]+c[i]-c[i-1]){
					s[j] = ASCII_LOWER[i+3];
				}
				for j in (x[i-1] + c[i] - c[i-1])..x[i] {
					s[j] = chars[curr];
					curr += 1;
					curr %= 3;
				}
			}
		}
        if possible {
            let ans : String = s.into_iter().collect();
            println!("YES");
            println!("{}", ans );
        }else{
            println!("NO");
        }
    }
}
