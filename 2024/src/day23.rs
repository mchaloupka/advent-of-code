use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type NodeIndices<'a> = HashMap<&'a str, usize>;

fn parse_input<'a>(input: &'a str) -> Graph<'a> {
    let mut output: Graph<'a> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once("-").unwrap();

        output.entry(l).or_default().insert(r);
        output.entry(r).or_default().insert(l);
    }

    output
}

fn part_1(input: &str) {
    let graph = parse_input(input);

    let all_nodes: HashMap<&str, usize> = graph
        .keys()
        .enumerate()
        .map(|(idx, node)| (*node, idx))
        .collect();

    let mut output = 0;

    for (&i, &i_idx) in all_nodes.iter() {
        let i_connect = graph.get(&i).unwrap();

        for &j in i_connect {
            let &j_idx = all_nodes.get(&j).unwrap();

            if j_idx <= i_idx {
                continue;
            }

            let j_connect = graph.get(&j).unwrap();
            let both_connect = i_connect.intersection(j_connect);

            for &k in both_connect {
                let &k_idx = all_nodes.get(&k).unwrap();

                if k_idx <= j_idx {
                    continue;
                }

                if i.starts_with("t") || j.starts_with("t") || k.starts_with("t") {
                    output += 1;
                }
            }
        }
    }

    println!("Part 1: {}", output);
}

fn get_groups_with_extra_item<'a>(
    graph: &'a Graph,
    node_indices: &NodeIndices,
    smaller_groups: &[HashSet<&'a str>],
) -> Vec<HashSet<&'a str>> {
    let mut output = Vec::new();

    for group in smaller_groups {
        let &max_index_in_group = group
            .iter()
            .map(|node| node_indices.get(node).unwrap())
            .max()
            .unwrap();

        let node_connections: Vec<_> = group.iter().map(|node| graph.get(node).unwrap()).collect();

        let all_connected = node_connections
            .iter()
            .skip(1)
            .fold(node_connections[0].clone(), |acc, &set| {
                acc.intersection(set).cloned().collect()
            });

        for next_node in all_connected {
            let &next_idx = node_indices.get(next_node).unwrap();
            if next_idx <= max_index_in_group {
                continue;
            }

            let mut output_group = group.clone();
            output_group.insert(next_node);
            output.push(output_group);
        }
    }

    output
}

fn part_2(input: &str) {
    let graph = parse_input(input);

    let all_nodes: HashMap<&str, usize> = graph
        .keys()
        .enumerate()
        .map(|(idx, node)| (*node, idx))
        .collect();

    let mut cur_groups: Vec<_> = all_nodes
        .iter()
        .map(|(&node, _)| HashSet::from([node]))
        .collect();

    let mut prev_groups = cur_groups.clone();

    while !cur_groups.is_empty() {
        prev_groups = cur_groups.clone();
        cur_groups = get_groups_with_extra_item(&graph, &all_nodes, &cur_groups);
    }

    if prev_groups.len() != 1 {
        panic!("Unexpected outcome!");
    }

    let mut output_group: Vec<_> = prev_groups[0].iter().cloned().collect();
    output_group.sort();

    println!("Part 2: {}", output_group.join(","));
}

pub fn run(input: &str) {
    part_1(input);
    part_2(input);
}
