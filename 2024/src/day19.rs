use std::{char, collections::HashMap, str::FromStr};

#[derive(Debug)]
struct TrieNode {
    word: String,
    is_word: bool,
    next_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn empty() -> TrieNode {
        TrieNode {
            word: String::new(),
            is_word: false,
            next_nodes: HashMap::new(),
        }
    }
    fn empty_for_word(word: &str) -> TrieNode {
        let mut v = TrieNode::empty();
        v.word = String::from_str(word).unwrap();
        v
    }

    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            self.is_word = true;
        } else {
            let first_char = word.chars().next().unwrap();

            let next_trie = self.next_nodes.entry(first_char).or_insert_with(|| {
                let mut word = self.word.clone();
                word.push(first_char);
                TrieNode::empty_for_word(&word)
            });

            next_trie.insert(&word[1..]);
        }
    }
}

fn parse_input(input: &str) -> (TrieNode, Vec<&str>) {
    let towels: Vec<_> = input
        .lines()
        .take(1)
        .flat_map(|line| line.split(",").map(|towel| towel.trim()))
        .collect();

    let mut trie = TrieNode::empty();

    for towel in towels {
        trie.insert(towel);
    }

    let patterns: Vec<_> = input.lines().skip(2).collect();

    (trie, patterns)
}

fn count_designs<'a>(
    pattern: &'a str,
    cur_trie: &'a TrieNode,
    root_trie: &'a TrieNode,
    cache: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    let cache_key = (pattern, cur_trie.word.as_str());

    match cache.get(&cache_key) {
        Some(x) => *x,
        None => {
            let mut outcome = 0;

            if cur_trie.is_word {
                if pattern.is_empty() {
                    outcome += 1;
                } else {
                    outcome += count_designs(pattern, root_trie, root_trie, cache);
                }
            }

            if !pattern.is_empty() {
                let first_char = pattern.chars().next().unwrap();

                if let Some(next_node) = cur_trie.next_nodes.get(&first_char) {
                    outcome += count_designs(&pattern[1..], next_node, root_trie, cache);
                }
            }

            cache.insert(cache_key, outcome);
            outcome
        }
    }
}

pub fn run(input: &str) {
    let (towels, patterns) = parse_input(input);
    let mut possible = 0;
    let mut total_designs = 0;

    let mut cache = HashMap::new();

    for pattern in patterns {
        let designs = count_designs(pattern, &towels, &towels, &mut cache);

        total_designs += designs;

        if designs > 0 {
            possible += 1;
        }
    }

    println!("Part 1: {}", possible);
    println!("Part 2: {}", total_designs);
}
