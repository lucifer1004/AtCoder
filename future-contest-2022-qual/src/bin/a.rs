use std::collections::{BinaryHeap, VecDeque};
use std::time::Instant;

use rand::prelude::*;

const MAX_ITER_TIMES: usize = 10;
const RECALCULATE_PERIOD: i32 = 20;
const GENETIC_RECALCULATE_PERIOD: i32 = 100;
const LEARNING_RATE: f64 = 0.8;
const MAX_POSTPONE: i32 = 7;
const MAX_WEIGHT: usize = 1_000_000_000_000_000_000;
const INF: f64 = 1e9;
const SKILL_LOWER: f64 = 1.0;
const SKILL_UPPER: f64 = 80.0;
const TIME_LIMIT_MILLI: u128 = 2700;
const MUTATION_RANGE: f64 = 3.0;
const CROSSOVER_PROBABILITY: f64 = 0.85;
const MUTATION_PROBABILITY: f64 = 0.01;
const POPULATION_SIZE: usize = 48;

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

fn topological_sort(adj: &[Vec<usize>], in_deg: &[usize]) -> Vec<usize> {
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

#[derive(Debug, Clone)]
struct Individual {
    skills: Vec<f64>,
    fitness: f64,
}

impl Individual {
    fn new(skills: Vec<f64>, tasks: &[Vec<f64>], records: &[(usize, f64)]) -> Self {
        let fitness = fitness(&skills, tasks, records);
        Self { skills, fitness }
    }

    fn cross_over(
        &self,
        other: &Individual,
        tasks: &[Vec<f64>],
        records: &[(usize, f64)],
    ) -> (Self, Self) {
        let n = self.skills.len();
        let mut rng = thread_rng();

        let mut daughter_dna = vec![];
        let mut son_dna = vec![];

        for i in 0..n {
            if rng.gen_range(0.0, 1.0) < 0.5 {
                daughter_dna.push(self.skills[i]);
                son_dna.push(other.skills[i]);
            } else {
                son_dna.push(self.skills[i]);
                daughter_dna.push(other.skills[i]);
            }
        }

        let daughter = Individual::new(daughter_dna, tasks, records);
        let son = Individual::new(son_dna, tasks, records);
        (daughter, son)
    }

    fn mutate(&mut self, tasks: &[Vec<f64>], records: &[(usize, f64)]) {
        let mut rng = thread_rng();
        let i = rng.gen_range(0, self.skills.len());
        self.skills[i] += rng.gen_range(-MUTATION_RANGE, MUTATION_RANGE);
        self.fitness = fitness(&self.skills, tasks, records);
    }
}

fn fitness(skills: &[f64], tasks: &[Vec<f64>], records: &[(usize, f64)]) -> f64 {
    let mut dist = 0.0;
    for &(task_idx, delta) in records.iter() {
        let estimated_delta = (0..skills.len())
            .map(|skill_idx| (tasks[task_idx][skill_idx] - skills[skill_idx]).max(0.0))
            .sum::<f64>();
        dist += (estimated_delta - delta) * (estimated_delta - delta);
    }

    dist = (dist / records.len() as f64).sqrt();

    1.0 / dist
}

fn random_population(
    population_size: usize,
    reference: &[f64],
    tasks: &[Vec<f64>],
    records: &[(usize, f64)],
) -> Vec<Individual> {
    let mut individuals: Vec<Individual> = Vec::new();

    for _ in 0..population_size {
        let skills = random_dna(reference);
        let indiv = Individual {
            fitness: fitness(&skills, tasks, records),
            skills,
        };
        individuals.push(indiv);
    }
    individuals
}

fn random_dna(reference: &[f64]) -> Vec<f64> {
    let mut v = reference.to_vec();
    for vi in v.iter_mut() {
        *vi += thread_rng().gen_range(-MUTATION_RANGE, MUTATION_RANGE);
    }
    v.to_vec()
}

fn find_fittest(population: &[Individual]) -> Individual {
    let mut best_individual = &population[0];

    for individual in population {
        if best_individual.fitness > individual.fitness {
            best_individual = individual;
        }
    }

    best_individual.clone()
}

#[derive(Debug, Clone)]
struct Simulation {
    fitness: f64,
    skills: Vec<f64>,
    iterations: usize,
    population_size: usize,
    crossover_probability: f64,
    mutation_probability: f64,
}

impl Simulation {
    fn new(skill_num: usize, iterations: usize) -> Self {
        Self {
            fitness: 0.0,
            skills: vec![0.0; skill_num],
            iterations,
            population_size: POPULATION_SIZE,
            crossover_probability: CROSSOVER_PROBABILITY,
            mutation_probability: MUTATION_PROBABILITY,
        }
    }

    fn run(&mut self, reference: &[f64], tasks: &[Vec<f64>], records: &[(usize, f64)]) {
        let mut population = random_population(self.population_size, reference, tasks, records);
        let mut champion = find_fittest(&population);

        for _ in 0..self.iterations {
            let challenger = find_fittest(&population);
            population = self.generate_population(population, tasks, records);

            if champion.fitness <= challenger.fitness {
                champion = challenger;
            }
        }
        self.fitness = champion.fitness;
        self.skills = champion.skills;
    }

    fn generate_population(
        &mut self,
        individuals: Vec<Individual>,
        tasks: &[Vec<f64>],
        records: &[(usize, f64)],
    ) -> Vec<Individual> {
        assert_eq!(
            self.population_size % 2,
            0,
            "population_size:{} should be divisible by 2",
            self.population_size
        );

        let cumulative_weights = get_cumulative_weights(&individuals);
        let mut next_population = Vec::new();

        for _ in 0..(self.population_size / 2) {
            // generate two individuals per iteration

            let (mom, dad) = select_parents(&cumulative_weights, &individuals);
            let (mut daughter, mut son) = self.generate_children(&mom, &dad, tasks, records);
            self.might_mutate_child(&mut daughter, tasks, records);
            self.might_mutate_child(&mut son, tasks, records);

            next_population.push(daughter);
            next_population.push(son);
        }
        next_population
    }

    fn generate_children(
        &mut self,
        mom: &Individual,
        dad: &Individual,
        tasks: &[Vec<f64>],
        records: &[(usize, f64)],
    ) -> (Individual, Individual) {
        if thread_rng().gen_bool(self.crossover_probability) {
            mom.cross_over(dad, tasks, records)
        } else {
            (mom.clone(), dad.clone())
        }
    }

    fn might_mutate_child(
        &mut self,
        child: &mut Individual,
        tasks: &[Vec<f64>],
        records: &[(usize, f64)],
    ) {
        if thread_rng().gen_bool(self.mutation_probability) {
            child.mutate(tasks, records);
        }
    }
}

fn get_cumulative_weights(individuals: &[Individual]) -> Vec<f64> {
    let mut running_sum = 0.0;
    let mut cumulative_weights = vec![running_sum];

    for i in individuals {
        running_sum += i.fitness;
        cumulative_weights.push(running_sum);
    }
    cumulative_weights
}

fn select_parents<'a>(
    w: &[f64],
    individuals: &'a [Individual],
) -> (&'a Individual, &'a Individual) {
    let mom_index = select_index(w);
    let dad_index = select_index(w);
    (&individuals[mom_index], &individuals[dad_index])
}

fn select_index(cumulative_weights: &[f64]) -> usize {
    // TODO: Error Handling
    let w_sum = cumulative_weights.last().unwrap();
    let r: f64 = thread_rng().gen_range(0.0, *w_sum);
    cumulative_weights.iter().rposition(|&w| w < r).unwrap()
}

fn main() {
    let now = Instant::now();
    let mut rng = rand::thread_rng();

    let params = read_line::<usize>();
    let task_num = params[0];
    let member_num = params[1];
    let skill_num = params[2];
    let dep_num = params[3];
    let mut tasks = vec![];
    let mut task_tendencies = vec![];
    let mut sum_difficulties = vec![];
    let mut max_difficulties = vec![];
    for i in 0..task_num {
        tasks.push(read_line::<f64>());
        let mut sum_difficulty: f64 = 0.0;
        let mut max_difficulty: f64 = 0.0;
        for &skill in tasks[i].iter() {
            sum_difficulty += skill;
            max_difficulty = max_difficulty.max(skill);
        }
        sum_difficulties.push(sum_difficulty);
        max_difficulties.push(max_difficulty);
        task_tendencies.push(
            tasks[i]
                .iter()
                .map(|&x| x / sum_difficulty)
                .collect::<Vec<_>>(),
        );
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

    let order = topological_sort(&adj, &in_deg);
    let mut weight = vec![0usize; task_num];
    for k in (0..task_num).rev() {
        let i = order[k];
        for &j in adj[i].iter() {
            weight[i] += weight[j];
            weight[i] = weight[i].min(MAX_WEIGHT);
        }
        // weight[i] = weight[i] * 3 / 2;
        weight[i] += adj[i].len() + 100 - max_difficulties[i].trunc() as usize;
        weight[i] = weight[i].min(MAX_WEIGHT);
    }

    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for i in 0..task_num {
        if in_deg[i] == 0 {
            pq.push((weight[i], i));
        }
    }

    // Estimated skill levels of each member.
    let mut estimate_skills = vec![vec![0.0; skill_num]; member_num];

    // Current job assigned to each member.
    let mut assignments: Vec<(usize, i32)> = vec![(task_num, 0); member_num];

    // Number of jobs a member has been assigned to.
    let mut assigned_times = vec![0usize; member_num];

    // Number of free members.
    let mut free_num = member_num;

    // Remaining chances to postpone a job.
    let mut remaining_postpone_chances = vec![MAX_POSTPONE; task_num];

    // Date when the member is supposed to complete the previous job.
    let mut estimate_free_date = vec![0i32; member_num];

    let mut records: Vec<Vec<(usize, f64)>> = vec![vec![]; member_num];

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

        // Update skill estimations via iteration.
        if day % RECALCULATE_PERIOD == 0 {
            for member_idx in 0..member_num {
                if records[member_idx].is_empty() {
                    continue;
                }

                let mut gradient = 1.0;
                let mut skills = estimate_skills[member_idx].clone();
                let mut current_fitness = fitness(&skills, &tasks, &records[member_idx]);

                for _ in 0..MAX_ITER_TIMES {
                    records[member_idx].shuffle(&mut rng);
                    for &(task_idx, delta) in records[member_idx].iter() {
                        let estimated_delta = (0..skill_num)
                            .map(|skill_idx| {
                                (tasks[task_idx][skill_idx] - skills[skill_idx]).max(0.0)
                            })
                            .sum::<f64>();
                        if estimated_delta > delta {
                            let need_to_inc = (estimated_delta - delta) * gradient;
                            let mut cnt = 0.0;
                            for skill_idx in 0..skill_num {
                                if skills[skill_idx] < tasks[task_idx][skill_idx] {
                                    cnt += 1.0;
                                }
                            }
                            for skill_idx in 0..skill_num {
                                if skills[skill_idx] < tasks[task_idx][skill_idx] {
                                    skills[skill_idx] += need_to_inc / cnt;
                                }
                                skills[skill_idx] = skills[skill_idx].min(SKILL_UPPER);
                            }
                        } else {
                            let need_to_dec = (delta - estimated_delta) * gradient;
                            let mut cnt = 0.0;
                            for skill_idx in 0..skill_num {
                                if skills[skill_idx] < tasks[task_idx][skill_idx] {
                                    cnt += 1.0;
                                }
                            }
                            for skill_idx in 0..skill_num {
                                if skills[skill_idx] < tasks[task_idx][skill_idx] {
                                    skills[skill_idx] -= need_to_dec / cnt;
                                }
                                skills[skill_idx] = skills[skill_idx].max(SKILL_LOWER);
                            }
                        }
                    }

                    gradient *= LEARNING_RATE;
                    let new_fitness = fitness(&skills, &tasks, &records[member_idx]);
                    if new_fitness > current_fitness {
                        estimate_skills[member_idx] = skills.clone();
                        current_fitness = new_fitness;
                    } else {
                        skills = estimate_skills[member_idx].clone();
                    }
                }

                if now.elapsed().as_millis() <= TIME_LIMIT_MILLI
                    && records[member_idx].len() >= 5
                    && day % GENETIC_RECALCULATE_PERIOD == 0
                {
                    let mut simulation =
                        Simulation::new(skill_num, (records[member_idx].len() * 20).min(200));
                    simulation.run(&estimate_skills[member_idx], &tasks, &records[member_idx]);
                    if simulation.fitness > current_fitness {
                        estimate_skills[member_idx] = simulation.skills;
                    }
                }
            }
        }

        for &member_idx in freed.iter() {
            let (task_idx, assigned_day) = assignments[member_idx];

            for &v in adj[task_idx].iter() {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    pq.push((weight[v], v));
                }
            }

            let delta = day - assigned_day - 1;
            records[member_idx].push((task_idx, delta as f64));
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
                let mut delta = 0.0;
                for i in 0..skill_num {
                    delta += (tasks[task_idx][i] - estimate_skills[member_idx][i]).max(0.0);
                }

                if assignments[member_idx].0 == task_num {
                    if delta < best_delta {
                        best_delta = delta;
                        best_member = member_idx;
                    }
                } else if remaining_postpone_chances[task_idx] > 0
                    && day + remaining_postpone_chances[task_idx] >= estimate_free_date[member_idx]
                {
                    best_delta_not_free = delta;
                }
            }

            if remaining_postpone_chances[task_idx] > 0
                && best_delta_not_free + (remaining_postpone_chances[task_idx] as f64) < best_delta
            {
                remaining_postpone_chances[task_idx] -= 1;
                postponed.push((weight[task_idx], task_idx));
                continue;
            }

            assignments[best_member] = (task_idx, day);
            estimate_free_date[best_member] = day + best_delta.trunc() as i32;
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
