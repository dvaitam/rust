use std::io;
fn decode(anb :bool, anc: bool, _bnc :bool, aob: bool, aoc :bool, boc :bool) -> (bool, bool, bool) {
	if anb {
		if anc {
			return (true, true, true)
		} else {
			return (true, true, false)
		}
	} else {
		if aob {
			if aoc {
				if anc {
					return (true, false, true)
				} else {
					if boc {
						return (false, true, true)
					} else {
						return (true, false, false)
					}
				}

			} else {
				return (false, true, false)
			}
		} else {
			if aoc {
				return (false, false, true)
			} else {
				return (false, false, false)
			}
		}
	}
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
    let (n, k) : (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    println!("and 1 2");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let anb : usize = line.trim().parse().expect("");
    println!("and 1 3");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let anc : usize= line.trim().parse().expect("");
    println!("and 2 3");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let bnc : usize= line.trim().parse().expect("");
    println!("or 1 2");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let aob :usize= line.trim().parse().expect("");
    println!("or 1 3");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let aoc : usize = line.trim().parse().expect("");
    println!("or 2 3");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let boc : usize = line.trim().parse().expect("");
    let mut a : Vec<usize> = vec![0;n];
    for j in 0..30 {
		let r = 1 << j;
		let (ai, bi, ci) = decode(anb&r == r, anc&r == r, bnc&r == r, aob&r == r, aoc&r == r, boc&r == r);
		if ai {
			a[0] += r;
		}
		if bi {
			a[1] += r;
		}
		if ci {
			a[2] += r;
		}
	}
    for i in 3..n {
        println!("and 1 {}", i+1);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let ni : usize = line.trim().parse().expect("");
        println!("or 1 {}", i+1);
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let oi : usize = line.trim().parse().expect("");
        for j in 0..30 {
			let r = 1 << j;
			if a[0]&r == r {
				a[i] += ni & r;
			} else {
				a[i] += oi & r;
			}
		}
    }
    a.sort();
    println!("finish {}", a[k-1]);


}