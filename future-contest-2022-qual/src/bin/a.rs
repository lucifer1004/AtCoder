use std::collections::{BinaryHeap, VecDeque};

const MAX_POSTPONE: i32 = 7;
const MAX_WEIGHT: usize = 1_000_000_000_000_000_000;
const INF: i32 = 1_000_000_000;

fn read_line<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().split(' ').map(|x| x.parse::<T>().unwrap()).collect::<Vec<_>>()
}

fn topo_sort(adj: &[Vec<usize>], in_deg: &[usize]) -> Vec<usize> {
    let mut deg = in_deg.to_owned();
    let n = deg.len();
    let mut order = vec![];
    let mut dq = VecDeque::new();
    for (i, &d) in deg.iter().enumerate().take(n) {
        if d == 0 {
            dq.push_back(i);
        }
    }
    while !dq.is_empty() {
        let u = dq.pop_front().unwrap();
        order.push(u);
        for &v in adj[u].iter() {
            deg[v] -= 1;
            if deg[v] == 0 {
                dq.push_back(v);
            }
        }
    }

    order
}

fn main() {
    let params = read_line::<usize>();
    let task_num = params[0];
    let member_num = params[1];
    let skill_num = params[2];
    let dep_num = params[3];
    let mut tasks = vec![];
    for _ in 0..task_num {
        tasks.push(read_line::<i32>());
    }
    let mut deps = vec![];
    for _ in 0..dep_num {
        let params = read_line::<usize>();
        deps.push((params[0] - 1, params[1] - 1));
    }

    let mut adj = vec![vec![]; task_num];
    let mut in_deg = vec![0usize; task_num];
    for &(u, v) in deps.iter() {
        adj[u].push(v);
        in_deg[v] += 1;
    }

    let order = topo_sort(&adj, &in_deg);
    let mut weight = vec![0usize; task_num];
    for k in (0..task_num).rev() {
        let i = order[k];
        for &j in adj[i].iter() {
            weight[i] += weight[j] * 3 / 2;
            weight[i] = weight[i].min(MAX_WEIGHT);
        }
        weight[i] += adj[i].len();
        weight[i] = weight[i].min(MAX_WEIGHT);
    }

    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for i in 0..task_num {
        if in_deg[i] == 0 {
            pq.push((weight[i], i));
        }
    }

    // Estimated skill levels of each member.
    let mut estimate_skills = vec![vec![0; skill_num]; member_num];

    // Current job assigned to each member.
    let mut assignments = vec![(task_num, 0); member_num];

    // Number of jobs a member has been assigned to.
    let mut assigned_times = vec![0usize; member_num];

    // Number of free members.
    let mut free_num = member_num;

    // Remaining chances to postpone a job.
    let mut remaining_postpone_chances = vec![MAX_POSTPONE; task_num];

    // Date when the member is supposed to complete the previous job.
    let mut estimate_free_date = vec![0i32; member_num];

    // Current date
    let mut day = 0;
    loop {
        let mut freed = vec![];

        if day >= 1 {
            let params = read_line::<i32>();
            if params[0] == -1 {
                break;
            }
            free_num += params[0] as usize;
            for &member_idx in params.iter().take(params[0] as usize + 1).skip(1) {
                freed.push((member_idx - 1) as usize);
            }
        }

        day += 1;

        for &member_idx in freed.iter() {
            let (task_idx, assigned_day) = assignments[member_idx];

            for &v in adj[task_idx].iter() {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    pq.push((weight[v], v));
                }

                let delta = day - assigned_day - 1;
                for i in 0..skill_num {
                    let original = estimate_skills[member_idx][i];
                    let revision = tasks[task_idx][i] - delta;
                    estimate_skills[member_idx][i] = original.max(revision);
                }
            }
            assignments[member_idx] = (task_num, 0);
        }

        let mut new_assignments: Vec<(usize, usize)> = vec![];
        let mut postponed = vec![];

        while free_num > 0 && !pq.is_empty() {
            let (_weight, task_idx) = pq.pop().unwrap();
            let mut best_member = member_num;
            let mut best_delta = INF;
            let mut best_delta_not_free = INF;
            for member_idx in 0..member_num {
                let mut delta = 0;
                for i in 0..skill_num {
                    delta += (tasks[task_idx][i] - estimate_skills[member_idx][i]).max(0);
                }
                if assignments[member_idx].0 == task_num {
                    if delta < best_delta {
                        best_delta = delta;
                        best_member = member_idx;
                    }
                } else if remaining_postpone_chances[task_idx] > 0 && day + remaining_postpone_chances[task_idx] >= estimate_free_date[member_idx] {
                    best_delta_not_free = delta;
                }
            }

            if remaining_postpone_chances[task_idx] > 0 && best_delta_not_free + remaining_postpone_chances[task_idx] < best_delta {
                remaining_postpone_chances[task_idx] -= 1;
                postponed.push((weight[task_idx], task_idx));
                continue;
            }

            assignments[best_member] = (task_idx, day);
            estimate_free_date[best_member] = day + best_delta;
            assigned_times[best_member] += 1;
            new_assignments.push((best_member, task_idx));
            free_num -= 1;
        }

        for &task in postponed.iter() {
            pq.push(task);
        }

        print!("{}", new_assignments.len());
        for &(member_idx, task_idx) in new_assignments.iter() {
            print!(" {} {}", member_idx + 1, task_idx + 1);
        }
        println!();
    }
}

