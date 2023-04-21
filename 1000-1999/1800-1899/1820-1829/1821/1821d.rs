use std::io;
use std::cmp::min;
use std::cmp::max;

#[derive(Copy, Clone)]
struct Segment{
	l : usize,
	r : usize
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t : i32 = line.trim().parse().expect(" not int");
    for _t in 0..t{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (n, mut k): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
    
        let l : Vec<usize> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
    
        let r : Vec<usize> = line.trim().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        let mut ones : Vec<usize> = Vec::new();
        let mut ones_not_used : Vec<usize> = Vec::new();
        let mut others : Vec<Segment> = Vec::new();

       
        for i in 0..n{
            if l[i] == r[i] {
                ones_not_used.push(l[i]);
            } else if k > 0 {
                if r[i]-l[i]+1 <= k {
                    others.push( Segment{l: l[i], r: r[i]});
                    k -= r[i] - l[i] + 1;
                } else {
                    others.push(Segment{l: l[i], r: r[i] - ((r[i] - l[i] + 1) - k)});
                    k = 0;
                }
            } else {
                break;
            }
        }
        while k > 0 && ones_not_used.len() >= k {
            ones = ones_not_used[..k].to_vec();
            ones_not_used = ones_not_used[k..].to_vec();
            k = 0;
        }
        if k > 0 {
            println!("-1");
            continue;
        }
        if others.len() == 0 {
			let mut ending = 0;

			if ones.len() > 0 {
				ending = max(ending, ones[ones.len()-1]);
			}
            println!("{}", ending+2*ones.len());
			continue;
		}
		let mut ending = others[others.len()-1].r;
		if ones.len() > 0 {
			ending = max(ending, ones[ones.len()-1])
		}
		let mut ans = ending + 2*(ones.len()+others.len());
		let mut curr = others.len() - 1;
		let mut start = 0;
		while  start < ones_not_used.len() {
			if others[curr].l < ones_not_used[start] {
				break;
			}
			start += others[curr].r - others[curr].l + 1;
			if start-1 >= ones_not_used.len() {
				break;
			}
			let mut new_ending = ones_not_used[start-1];
			if curr >= 1 {
				new_ending = max(new_ending, others[curr-1].r);
			}
			ans = min(ans, new_ending+curr*2+ones.len()*2+start*2);
            if curr == 0 {
                break;
            }
			curr -= 1;
		}
		println!("{}", ans);
    }
   
}
