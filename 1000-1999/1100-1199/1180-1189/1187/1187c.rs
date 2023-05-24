use std::cmp::max;
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
    let mut split = line.trim().split(" ");
    let (n, m): (usize, usize) = (
        split.next().unwrap().parse().expect("not int"),
        split.next().unwrap().parse().expect("not int"),
    );

    let mut sorted: Vec<Segment> = Vec::new();
    let mut unsorted: Vec<Segment> = Vec::new();

    for _i in 0..m {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (e, l, r): (usize, usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        if e == 1 {
            sorted.push(Segment { l: l - 1, r: r - 1 })
        } else {
            unsorted.push(Segment { l: l - 1, r: r - 1 })
        }
    }
    sorted.sort_unstable_by_key(|item| (item.l, item.r));

    let mut uniq: Vec<Segment> = Vec::new();
    for i in 0..sorted.len() {
        if uniq.len() == 0 {
            uniq.push(sorted[i])
        } else {
            let ll = uniq.len();
            if uniq[ll - 1].r >= sorted[i].l {
                uniq[ll - 1].r = max(uniq[ll - 1].r, sorted[i].r);
            } else {
                uniq.push(sorted[i])
            }
        }
    }
    let mut ans: Vec<usize> = vec![0; n];
    let mut curr = 2000;
    let mut prev = 0;
    for i in 0..uniq.len() {
        let (l, r) = (uniq[i].l, uniq[i].r);
        for j in prev..l {
            ans[j] = curr;
            curr -= 1;
        }
        curr -= 1;
        for j in l..r + 1 {
            ans[j] = curr;
        }
        curr -= 1;
        prev = r + 1;
    }
    for i in 0..n {
        if ans[i] == 0 {
            ans[i] = curr;
            curr -= 1;
        }
    }
    let mut ok = true;

    for i in 0..unsorted.len() {
        let (l, r) = (unsorted[i].l, unsorted[i].r);
        let mut good = false;
        for j in l..r + 1 {
            if j + 1 <= r {
                if ans[j] <= ans[j + 1] {
                    continue;
                } else {
                    good = true;
                    break;
                }
            }
        }
        if !good {
            ok = false;
            break;
        }
    }
    if ok {
        println!("YES");
        for i in 0..ans.len() {
            print!("{} ", ans[i])
        }
        println!();
    } else {
        println!("NO");
    }
}
