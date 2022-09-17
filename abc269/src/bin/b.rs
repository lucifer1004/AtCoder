use proconio::input;

fn main() {
    input! {
        s: [String; 10],
    }

    let s = s
        .iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut a = 9;
    let mut b = 0;
    let mut c = 9;
    let mut d = 0;
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = a.min(i);
                b = b.max(i);
                c = c.min(j);
                d = d.max(j);
            }
        }
    }
    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
