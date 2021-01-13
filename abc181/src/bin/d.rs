use proconio::input;

fn str_to_cnt(s: String) -> Vec<u32> {
    let s: Vec<u32> = s
        .chars()
        .into_iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut cnt = vec![0; 10];
    for i in s {
        cnt[i as usize] += 1;
    }
    cnt
}

fn main() {
    input! {
        s: String,
    }

    let min_len = s.len();
    let have = str_to_cnt(s);

    for i in (8..1000).step_by(8) {
        let need = str_to_cnt(format!("{:0>1$}", i.to_string(), std::cmp::min(min_len, 3)));
        let mut ok = true;
        for j in 0..10 {
            if need[j] > have[j] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
