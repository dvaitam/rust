use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
fn prime_factors(mut n : i64, primes: &Vec<i64> ) -> HashMap<i64, i32> {
    let mut ans : HashMap<i64, i32> = Default::default();
	while n > 1 {
		for i in 0..primes.len(){
            if primes[i] * primes[i] > n {
                let count = ans.entry(n).or_insert(0);
                *count += 1;
                n = 1;
                break
            }
			if n%primes[i] == 0 {
                let count = ans.entry(primes[i]).or_insert(0);
                *count += 1;
				n = n / primes[i];
				break
			}
		}
	}
	return ans;
}
fn pow(x : i64, n : i32) -> i64 {
    let mut ans = 1;
    let mut n = n;
    while n > 0 {
        ans = ans * x;
        n -= 1;
    }
    return ans
}
fn main() {
    let mut primes : Vec<i64> = Vec::new();
    primes.push(2);
	for start in 3..1000 {
		let mut ok = true;
		for i in 0..primes.len() {
			if start%primes[i] == 0 {
				ok = false;
				break;
			}
		}
		if ok {
            primes.push(start);
		}
	}
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut split = line.trim().split(" ");
   
    let (n, k): (usize, i32) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let a : Vec<i64> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut m : HashMap<i64, i64> = HashMap::new();
    let mut invm : HashMap<i64, i64> = HashMap::new();
    for i in 0..n{
        let factors = prime_factors(a[i], &primes);
        let mut val : i64 = 1;
        let mut inv : i64 = 1;
       
        for (key, v) in factors {
            let v = v % k;
            val *= pow(key, v);
            if v > 0 {
                inv *= pow(key, k-v);
            }
        }
        invm.entry(val).or_insert(inv);
        let count = m.entry(val).or_insert(0);
        *count += 1;
    }
    let mut ans : i64 = 0;
    let mut counted : HashSet<i64> = HashSet::new();
	for (key, v) in  &m {
		if counted.contains(&key) {
			continue;
		}
		if *key == invm[&key] {
			ans += (v * (v - 1)) / 2;
		} else {
            if m.contains_key(&invm[&key]) {
                ans += v * m[&invm[&key]];

            }
		}
        counted.insert(invm[&key]);
	}
    println!("{}", ans);
    
}
