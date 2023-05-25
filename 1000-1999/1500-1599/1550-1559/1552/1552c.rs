use std::collections::HashSet;
use std::io;
#[derive(Copy, Clone)]
struct Chord {
    p1: usize,
    p2: usize,
}
fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let t: u32 = line.trim().parse().expect("not a number");
    for _t in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let mut split = line.trim().split(" ");
        let (n, k): (usize, usize) = (
            split.next().unwrap().parse().expect("not int"),
            split.next().unwrap().parse().expect("not int"),
        );
        let mut chords: Vec<Chord> = vec![Chord { p1: 0, p2: 0 }; k];
        let mut h: HashSet<usize> = HashSet::new();
        for i in 0..k {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let mut split = line.trim().split(" ");
            let (p1, p2): (usize, usize) = (
                split.next().unwrap().parse().expect("not int"),
                split.next().unwrap().parse().expect("not int"),
            );
            h.insert(p1);
            h.insert(p2);
            if p1 < p2 {
                chords[i] = Chord { p1: p1, p2: p2 };
            } else {
                chords[i] = Chord { p1: p2, p2: p1 };
            }
        }
        let mut rem: Vec<usize> = Vec::new();
        for i in 1..(2 * n + 1) {
            if !h.contains(&i) {
                rem.push(i);
            }
        }
        rem.sort();
        let m = rem.len();
        for i in 0..(m / 2) {
            chords.push(Chord {
                p1: rem[i],
                p2: rem[i + m / 2],
            });
        }
        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                let chord1 = chords[i];
                let chord2 = chords[j];
                if (chord1.p1 > chord2.p1 && chord1.p1 < chord2.p2 && chord1.p2 > chord2.p2)
                    || (chord1.p2 > chord2.p1 && chord1.p2 < chord2.p2 && chord1.p1 < chord2.p1)
                {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
