use std::io;
fn other(i :usize, j :usize) -> usize {
    let mut m : Vec<bool> = Vec::new();
    m.resize(3, false);
	m[i] = true;
	m[j] = true;
	for k in 0..3 {
		if !m[k] {
			return k;
		}
	}
	return 0;
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n : usize = line.trim().parse().expect("not int");
    

    let mut c : Vec<Vec<i64>> = Vec::new();

    for _i in 0..3 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        c.push(line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect());
    }
    let mut adj : Vec<Vec<usize>> = Vec::new();
    for _i in 0..n+1 {
        adj.push(Vec::new());
    }
    let mut ok : bool = true;
    for _i in 0..n-1 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (u, v): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        adj[v].push(u);
        adj[u].push(v);
        if adj[u].len() > 2 || adj[v].len() > 2 {
            ok = false
        }
    }
    if ok {
        let mut line : Vec<usize> = Vec::new();
		for i in 1..n+1{
			if adj[i].len() == 1 {
                line.push(i);
                line.push(adj[i][0]);
				break
			}
		}

		while adj[line[line.len()-1]].len() > 1 {
			let last = line[line.len()-1];
            for v in adj[last].iter() {
				if *v == line[line.len()-2] {
					continue;
				} else {
                    line.push(*v);
					break
				}
			}
		}
		let mut min_cost : i64 = -1;
		let (mut ans_i, mut ans_j) : (usize, usize)  = (0, 0);
		for i in 0..3 {
			for j in 0..3 {
				if i == j {
					continue;
				}
				let mut cost : i64 = 0;
				let  ( mut curr, mut nxt) : (usize, usize) = (i, j);
				for k in  line.iter() {
					cost += c[curr][k-1];
					(curr, nxt)= (nxt, other(curr, nxt));
				}
				if min_cost == -1 || cost < min_cost {
					min_cost = cost;
					(ans_i, ans_j) = (i, j);
				}
			}
		}
		let (mut curr, mut nxt) : (usize, usize) = (ans_i, ans_j);
        println!("{}", min_cost);
        let mut ans = vec![0; n+1];
		for k in line.iter() {
			ans[*k] = curr + 1;
			(curr, nxt) = (nxt, other(curr, nxt));
		}
		for i in 1..n+1 {
            print!("{} ", ans[i])
		}
		println!()
	} else {
		println!("-1")
	}
}
