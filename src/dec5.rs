type Rules = Vec<Vec<usize>>;

fn load(input: &str) -> (Rules, Vec<Vec<usize>>) {
    let mut dag: Rules = (0..100).map(|_| vec![]).collect();
    let (graph, input) = input.split_once("\n\n").unwrap();
    graph.lines()
       .map(|l| l.split_once('|').unwrap())
       .map(|(a, b)| (a.parse().unwrap(), b.parse::<usize>().unwrap()))
       .for_each(|(a, b)| dag[b].push(a));
    let inputs = input.lines().map(|line| line.split(",").map(|x| x.parse().unwrap()).collect()).collect();
    (dag, inputs)
}

pub fn part1(input: &str) -> usize {
    let (graph, inputs) = load(input);
    inputs.iter().filter(|nums| valid(&graph, nums)).map(|nums| nums[nums.len() / 2]).sum()
}

fn valid(dag: &Rules, nums: &Vec<usize>) -> bool {
    for (i, p) in nums.iter().enumerate() {
        if !dag[*p].iter().all(|r| !nums.contains(r) || nums[..i].contains(r)) {
            return false
        }
    }
    true
}

pub fn part2(input: &str) -> usize {
    let (graph, inputs) = load(input);
    inputs.iter().filter(|line| !valid(&graph, line)).map(|nums| {
        let mut nums = nums.clone();
        let mut i = 0;
        while i < nums.len() {
            if let Some(missing) = graph[nums[i]].iter().find(|r| nums.contains(r) && !&nums[..i].contains(r)) {
                let missing_idx = nums.iter().position(|x| x == missing).unwrap();
                nums.swap(i, missing_idx);
            } else {
                i += 1;
            }
        }
        nums
    }).map(|nums| nums[nums.len() / 2]).sum()
}