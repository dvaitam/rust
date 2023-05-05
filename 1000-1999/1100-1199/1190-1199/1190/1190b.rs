use std::io;
use std::collections::HashMap;
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let n : usize = line.trim().parse().expect("not int");
   
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let mut a : Vec<i32> = line.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    let mut m : HashMap<i32, i32> = HashMap::new();
    for i in 0..n{
        let count = m.entry(a[i]).or_insert(0);
        *count += 1;
    }
    let mut lost = false;
    let mut twice = false;
    let mut double = -1;
    for (key, v) in m {
        if v >= 3 {
            lost = true;
            break;
        }
        if v >= 2 && key == 0 {
            lost = true;
            break;
        }
        if twice && v >= 2 {
            lost = true;
            break
        }
        if v >= 2 {
            twice = true;
            double = key;
        }
    }
    if !lost {
        if twice {
            for i in 0..n {
                if a[i] == double-1 {
                    lost = true;
                    break;
                }
            }
            if !lost {
                for i in 0..n {
                    if a[i] == double {
                        a[i] -= 1;
                        break;
                    }
                }
            }
        }
    }
    if lost {
        println!("cslnb");
        return;
    }
    let mut curr = 0;
    let mut ans = 0;
    a.sort();
    for i in 0..n {
		if curr <= a[i] {
			ans += a[i] - curr;
			ans %= 2;
			curr += 1;
		}
	}
    if twice  {
        ans += 1;
        ans %= 2;
    }
    if ans % 2 == 0 {
        println!("cslnb");
    }else{
        println!("sjfnb");
    }
}
