use std::io;
use std::collections::HashSet;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n : usize = line.trim().parse().expect("not int");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let x : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut m : HashSet<i32> = HashSet::new();
    for i in 0..n{
        m.insert(x[i]);
    }
    let mut ans : Vec<i32> = vec![0;1];
	ans[0] = x[0];
	for i in 0..n {
		if ans.len() == 3 {
			break;
		}

		for j in 0..31 {
			let curr = 1 << j;
			if ans.len() < 3 {
				if m.contains(&(x[i]+curr)) && m.contains(&(x[i]+2*curr)) {
					let mut tmp : Vec<i32> = Vec::new();
					tmp.push(x[i]);
					tmp.push(x[i]+curr);
					tmp.push(x[i]+2*curr);
					ans = tmp;
				} else {
					if ans.len() < 2 {
						if m.contains(&(x[i]+curr)) {
							let mut tmp : Vec<i32> = Vec::new();
                            tmp.push(x[i]);
                            tmp.push(x[i]+curr);
                            ans = tmp;
						}
					}
				}
			} else {
				break;
			}

		}
	}
    println!("{}", ans.len());
    for i in 0..ans.len(){
        print!("{} ", ans[i]);
    }
    println!();
    
}
