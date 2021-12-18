fn read_str() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}

fn read_line<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_str()
        .trim()
        .split(' ')
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

const N: usize = 20;

fn dfs(u: usize, d: usize, adj: &Vec<Vec<Vec<usize>>>, vis: &mut Vec<bool>, ans: &mut Vec<char>) {
    for i in 0..4 {
        let nd = (d + i) % 4;
        if !adj[u][nd].is_empty() {
            let v = adj[u][nd][0];
            if !vis[v] {
                ans.push('F');
                vis[v] = true;
                dfs(v, nd, adj, vis, ans);
                for j in 0..2 {
                    let nnd = (nd + j) % 4;
                    if adj[v][nnd].is_empty() {
                        ans.push('R');
                    } else {
                        ans.push('r');
                    }
                }
                ans.push('F');
            }

            let nnd = (nd + 2) % 4;
            if adj[u][nnd].is_empty() {
                ans.push('L');
            } else {
                ans.push('l');
            }
        } else {
            ans.push('r');
        }
    }
}

fn main() {
    let s = read_line::<usize>();
    let si = s[0];
    let sj = s[1];
    let mut adj = vec![vec![vec![]; 4]; N * N];

    for row in 0..N {
        let str = read_str().chars().collect::<Vec<_>>();
        for col in 0..N - 1 {
            let u = row * N + col;
            let v = u + 1;
            if str[col] == '0' {
                adj[u][1].push(v);
                adj[v][3].push(u);
            }
        }
    }

    for row in 0..N - 1 {
        let str = read_str().chars().collect::<Vec<_>>();
        for col in 0..N {
            let u = row * N + col;
            let v = u + N;
            if str[col] == '0' {
                adj[u][2].push(v);
                adj[v][0].push(u);
            }
        }
    }

    let mut ans = vec![];
    let source = si * N + sj;
    let mut vis = vec![false; N * N];
    vis[source] = true;
    dfs(source, 0, &adj, &mut vis, &mut ans);

    let mut compressed = vec![];
    let mut counter = 0;
    let mut ch = '#';
    for &c in ans.iter() {
        if c == ch {
            counter += 1;
        } else {
            if counter > 1 {
                compressed.push(counter.to_string());
            }
            if counter >= 1 {
                compressed.push(ch.to_string());
            }
            counter = 1;
            ch = c;
        }
    }

    if counter > 1 {
        compressed.push(counter.to_string());
    }
    if counter >= 1 {
        compressed.push(ch.to_string());
    }

    println!("{}", compressed.join(""));
}
