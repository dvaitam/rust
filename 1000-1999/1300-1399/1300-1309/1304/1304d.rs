use std::io;
#[derive(Copy, Clone)]
struct Symbol{
    ch : char,
    count: usize
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
        let (_n, s) : (usize, Vec<char>) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().trim().chars().collect()
        );
        let mut ggt = 0;
        let mut symbols : Vec<Symbol> = Vec::new();
		for i in 0..s.len() {
			if s[i] == '>' {
				ggt += 1;
			}
			if symbols.len() == 0 {
                symbols.push(Symbol{ch:s[i], count:1});
			} else {
				let last = symbols[symbols.len()-1];
				if last.ch == s[i] {
                    let ll = symbols.len();
					symbols[ll-1].count += 1;
					//last.count++
				} else {
                    symbols.push(Symbol{ch:s[i], count:1});
				}
			}
		}
        let mut ans : Vec<usize> = Vec::new();
		let mut lst = 1;
		let mut gt = ggt;
		if s[0] == '>' {
			gt += 1;
		}
		for i in (0..symbols.len()).rev(){
			let mut count = symbols[i].count;
			if i == 0 {
				count += 1;
			}

			if symbols[i].ch == '>' {
				for _j in 0..count {
                    ans.push(lst);
					lst += 1;
				}
			} else {
				for j in 0..count {
                    ans.push(gt+count-j);
				}
				gt += symbols[i].count;
			}
		}
		for i in (0..ans.len()).rev() {
            print!("{} ", ans[i]);
		}
		println!();

		let mut gt = ggt;
		if s[0] == '>' {
			gt += 1;
		}
		let mut lt = gt + 1;
		for i in 0..s.len() {
			if i == 0 {
				if s[i] == '<' {
                    print!("{} {}", lt, lt+1);
					lt += 2;
				} else {
                    print!("{} {}", gt, gt-1);
					gt -= 2;
				}
			} else {
				if s[i] == '<' {
                    print!(" {}", lt);
					lt += 1;
				} else {
                    print!(" {}", gt);
					gt -= 1;
				}
			}
		}
		println!();
    }
}
