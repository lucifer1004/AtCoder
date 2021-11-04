# Batch processing tasks

```rust
fn main() {
    loop {
        // Batch assignments.
        if free_num >= 2 && pq.len() >= 2 {
            let free_members = (0..member_num).filter(|&member_idx| assignments[member_idx].0 == task_num).collect::<Vec<_>>();
            let mut batched_tasks = vec![];
            for _ in 0..2 {
                batched_tasks.push(pq.pop().unwrap().1);
            }

            let mut cost = vec![vec![0; batched_tasks.len()]; member_num];
            for &member_idx in free_members.iter() {
                for (i, &task_idx) in batched_tasks.iter().enumerate() {
                    cost[member_idx][i] = (0..skill_num).map(|j| (tasks[task_idx][j] - estimate_skills[member_idx][j]).max(0)).sum();
                }
            }

            let mut best_delta = INF;
            let mut best_combination = vec![];
            for &member_idx_1 in free_members.iter() {
                for &member_idx_2 in free_members.iter() {
                    if member_idx_1 == member_idx_2 {
                        continue;
                    }
                    let delta = cost[member_idx_1][0].max(cost[member_idx_2][1]);
                    if delta < best_delta {
                        best_delta = delta;
                        best_combination = vec![member_idx_1, member_idx_2];
                    }
                }
            }

            for i in 0..2 {
                new_assignments.push((best_combination[i], batched_tasks[i]));
                assignments[best_combination[i]] = (batched_tasks[i], day);
                estimate_free_date[best_combination[i]] = day + cost[best_combination[i]][i];
                assigned_times[best_combination[i]] += 1;
                free_num -= 1;
            }

            continue;
        }
    }
}

```

# Dynamic weights

```rust
fn main() {
    loop {
        let mut pq = remaining_tasks.iter()
            .filter(|&&task_idx| in_deg[task_idx] == 0)
            .map(|&task_idx| {
                let mut current_weight = weight[task_idx];
                let mut direct_links = 0;
                for &v in adj[task_idx].iter() {
                    if in_deg[v] == 1 {
                        direct_links += 1;
                    }
                }
                current_weight += direct_links * DIRECT_LINK_WEIGHT;
                (current_weight, task_idx)
            }).collect::<BinaryHeap<_>>();
    }
}
```

# Other ideas

- First time reward: no change
