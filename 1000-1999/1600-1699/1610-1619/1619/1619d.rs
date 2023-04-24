use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t : i32 = line.trim().parse().expect(" not int");
    let mut _t = 0;
    while _t < t{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            if line.ends_with('\n') {
                line.pop();
               // println!("popped newline");
            }
            if  line.ends_with('\r'){
                line.pop();
                //println!("popped tab");
        
            }
            if line.ends_with('\n') {
                line.pop();
              //  println!("popped newline");
            }
            if  line.ends_with('\r'){
                line.pop();
                //println!("popped tab");
        
            }
            
            if line.len() == 0 {
                continue;
            }else{
                _t += 1;
            }
        
        let mut split = line.trim().split(" ");
        let (m, n): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        

        let mut p : Vec<Vec<i32>> = vec![Vec::new() ; m];
        


        for i in 0..m{
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read line");
            p[i] = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        }
        let mut uniq : HashSet<i32> = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                uniq.insert(p[i][j]);
            }
        }
        let mut keys : Vec<i32>  = uniq.into_iter().collect();
        keys.sort();
        let (mut start, mut end) : (usize, usize) = (0, keys.len());

		while start < end {
			let mid = (start + end) / 2;

			let check = keys[mid];
			let mut counts : Vec<i32> = vec![0;n];
            let mut shops : HashMap<usize, i32> = HashMap::new();
			for i in 0..m {
				for j in 0..n {
					if p[i][j] >= check {
						counts[j] += 1;
                        let count = shops.entry(i).or_insert(0);
                        *count += 1;
					}
				}
			}
			let mut possible = true;
			for i in 0..n{
				if counts[i] == 0 {
					possible = false;
					break;
				}
			}
			if possible {
				if shops.len() > n-1 {
					let mut number_of_shops = shops.len();
                    let mut order : Vec<&usize> = shops.keys().into_iter().collect();
					order.sort_unstable_by_key(|item| (shops[item]));
					for i in 0..order.len() {
						let mut can_remove = true;
						for j in 0..n {
							if p[*order[i]][j] >= check && counts[j] == 1 {
								can_remove = false;
								break;
							}
						}
						if can_remove {
							for j in 0..n {
								if p[*order[i]][j] >= check {
									counts[j] -= 1;
								}
							}
							number_of_shops -= 1;
						}
						if number_of_shops <= n-1 {
							break;
						}
					}
					//	write(f, "number of shops", number_of_shops, check, "\n")
					if number_of_shops > n-1 {
						possible = false;
					}
				}
			}

			if possible {
				start = mid + 1;
			} else {
				end = mid;
			}
		}
        println!("{}", keys[start-1]);

    }
}
