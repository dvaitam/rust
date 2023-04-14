use std::io;
use std::collections::HashMap;
use std::collections::HashSet;
const MD : i64 = 998244353; 

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
fn ncr(n: i64, r: i64, primes: &Vec<i64>) -> i64 {
	if n == 0 {
		return 0;
	}
	if n < r {
		return 0;
	}
	if n == r {
		return 1;
	}
	if r == 0 {
		return 1;
	}
    if r > n/2  {
        return ncr(n, n-r, primes);
    }
    let mut nu : HashMap<i64, i32> = Default::default();

	let mut tmp = n;
    let mut d = r;
	while d > 0 {
		let m = prime_factors(tmp, primes);
		for (k, v) in  m {
            let count = nu.entry(k).or_insert(0);
            *count += v;
		}
        tmp -= 1;
        d -= 1;
	}
    let mut d = r;
	while d > 0 {
		let m = prime_factors(d, primes);
		for (k, v) in  m {
            let count = nu.entry(k).or_insert(0);
            *count -= v;
		}
        d -= 1;
	}
    let mut ans : i64 = 1;

	for (k, v) in nu {
		for _j in 0..v{
			ans = ans * k;
			ans = ans % MD;
		}
	}

	return ans;
}
fn get_xy(a : i64, b : i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let ( x1,  y1) = get_xy(b, a%b);
    let (x,y) = (y1, x1- y1*(a/b));
    return (x,y)
}
fn inverse(a :i64) -> i64 {
    let (x, y) = get_xy(a, MD);
    return (x % MD+MD) % MD;
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
    let (n, k) : (i64, i64) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );
    
    let mut ms : HashMap<i64, i64> = HashMap::new();
    let mut me : HashMap<i64, i64> = HashMap::new();
    let mut m : HashSet<i64> = HashSet::new();

    for _i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (l, r): (i64, i64) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let count = ms.entry(l).or_insert(0);
        *count += 1;
        let count = me.entry(r).or_insert(0);
        *count += 1;
        m.insert(l);
        m.insert(r);
    }
    let mut keys : Vec<i64> = m.into_iter().collect();
    keys.sort();
    let mut rem : i64 = 0;
    let mut ans : i64 = 0;

    let mut nc : Vec<i64> = Vec::new();
    nc.resize((n+1) as usize, 0);
		

	for i in 0..keys.len() {
        let nu = ms.entry(keys[i]).or_insert(0);
		if *nu > 0 {
            nc[(*nu+rem) as usize] += 1;
            nc[rem as usize] -= 1;
		}
		rem += *nu;
        let nu = me.entry(keys[i]).or_insert(0);
		rem -= *nu;
	}
    let mut fact : Vec<i64> = Vec::new();
    fact.resize((n+1) as usize, 0);
    fact[1] = 1;
    for i in 2..n+1 {
        fact[i as usize] = fact[(i-1) as usize] * i;
        fact[i as usize] %= MD;
    }
    // for i in 1..n+1 {
    //     print!("{} ", nc[i as usize]);
    // }
    // println!();
    for i in k..n+1 {
        if nc[i as usize] != 0 {
            if nc[i as usize] > 0 {
                let mut ncv = fact[i as usize];
                ncv = ncv * inverse(fact[k as usize]);
                ncv = ncv % MD;
                ncv = ncv * inverse(fact[(i-k) as usize]);
                ncv = ncv % MD;
                if i == k {
                    ncv = 1;
                }
                // println!("ncr {} {} {}", i, k,ncv);
                ans += nc[i as usize]*ncv;
                ans = ans % MD
            }else if nc[i as usize] < 0 {
                let mut ncv = fact[i as usize];
                ncv = ncv * inverse(fact[k as usize]);
                ncv = ncv % MD;
                ncv = ncv * inverse(fact[(i-k) as usize]);
                ncv = ncv % MD;
                if i == k {
                    ncv = 1;
                }
                // println!("ncr {} {} {}", i, k,ncv);
                ans += (MD+nc[i as usize])*ncv;
                ans = ans % MD
            }
        }
    }
	println!("{}", ans);    
}
