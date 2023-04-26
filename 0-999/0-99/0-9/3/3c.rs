use std::io;
fn has_won(s : &Vec<Vec<char>>) -> (bool, bool) {
	let (mut first_won, mut second_won) = (false, false);
	for i in 0..3 {
		let mut poss = true;
		for j in 1..3 {
			if s[i][j] != s[i][j-1] {
				poss = false;
				break;
			}
		}
		if poss {
			if s[i][0] == 'X' {
				first_won = true;
			} else if s[i][0] == '0' {
				second_won = true;
			}
		}
	}
	for j in 0..3 {
		let mut poss = true;
		for i in 1..3{
			if s[i][j] != s[i-1][j] {
				poss = false;
				break;
			}
		}
		if poss {
			if s[0][j] == 'X' {
				first_won = true;
			} else if s[0][j] == '0' {
				second_won = true;
			}
		}
	}
	let mut poss = true;
	for i in 1..3 {
		if s[i][i] != s[i-1][i-1] {
			poss = false;
		}
	}
	if poss {
		if s[0][0] == 'X' {
			first_won = true;
		} else if s[0][0] == '0' {
			second_won = true;
		}
	}
	poss = true;
	for i in 1..3{
		if s[i][2-i] != s[i-1][3-i] {
			poss = false;
		}
	}
	if poss {
		if s[0][2] == 'X' {
			first_won = true;
		} else if s[0][2] == '0' {
			second_won = true;
		}
	}
	return (first_won, second_won)
}
fn main() {
    let mut s : Vec<Vec<char>> = Vec::new();
    for _i in 0..3 {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        s.push(line.trim().to_string().chars().collect());
    }
    let mut c = 0;
	let mut n = 0;
	for i in 0..3 {
		for j in 0..3{
			if s[i][j] == 'X' {
				c += 1;
			} else if s[i][j] == '0' {
				n += 1;
			}
		}
	}
	if c-n > 1 || n > c {
        println!("illegal");
	} else {
		let (first_won, second_won) = has_won(&s);
		if first_won && second_won {
            println!("illegal");
		} else if first_won {
			if c == n {
                println!("illegal");
			} else {
                println!("the first player won");
			}
		} else if second_won {
			if c == n {
                println!("the second player won");
			} else {
                println!("illegal");
			}
		} else {
			if c == n {
                println!("first");
			} else if c+n != 9 {
                println!("second");
			} else {
                println!("draw");
			}
		}

	}
}