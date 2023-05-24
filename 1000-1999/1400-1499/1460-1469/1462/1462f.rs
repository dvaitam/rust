use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
#[derive(Copy, Clone)]
struct Segment {
    l: usize,
    r: usize,
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let n: usize = line.trim().parse().expect("not int");
        let mut segments: Vec<Segment> = vec![Segment { l: 0, r: 0 }; n];
        let mut starting: HashMap<usize, usize> = HashMap::new();
        let mut ending: HashMap<usize, usize> = HashMap::new();
        let mut uniq: HashSet<usize> = HashSet::new();
        for i in 0..n {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let mut split = line.trim().split(" ");
            let (l, r): (usize, usize) = (
                split.next().unwrap().parse().expect("not int"),
                split.next().unwrap().parse().expect("not int"),
            );
            segments[i] = Segment { l: l, r: r };
            let count = starting.entry(l).or_insert(0);
            *count += 1;
            let count = ending.entry(r).or_insert(0);
            *count += 1;
            uniq.insert(l);
            uniq.insert(r);
        }
        let mut uniqr: Vec<_> = uniq.into_iter().collect();
        uniqr.sort();
        let mut seen: Vec<usize> = vec![0; uniqr.len()];
        let mut unseen: Vec<usize> = vec![0; uniqr.len()];
        let mut index: HashMap<usize, usize> = HashMap::new();
        let mut started: usize = 0;
        let mut ended: usize = 0;
        for i in 0..uniqr.len() {
            if starting.contains_key(&uniqr[i]) {
                started += starting[&uniqr[i]];
            }
            seen[i] = started;
            if ending.contains_key(&uniqr[i]) {
                ended += ending[&uniqr[i]];
            }
            unseen[i] = ended;
            index.entry(uniqr[i]).or_insert(i);
        }
        let mut ans = n;
        for i in 0..n {
            let mut s = seen[index[&segments[i].r]];
            if index[&segments[i].l] > 0 {
                s -= unseen[index[&segments[i].l] - 1];
            }
            ans = min(ans, n - s);
        }
        println!("{}", ans);
    }
}
