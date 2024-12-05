use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_05").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut sum = 0;

    let mut rules_portion = true;
    for line in contents.lines() {
        if line.len() == 0 {
            rules_portion = false;
        } else if rules_portion {
            let mut iter = line.split('|');
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            rules.push((l, r));
        } else {
            let update = line
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            updates.push(update);
        }
    }

    'upate_loop: for update in updates.into_iter() {
        for rule in rules.iter() {
            let l_pos = update.iter().position(|&val| val == rule.0);
            let r_pos = update.iter().position(|&val| val == rule.1);

            match (l_pos, r_pos) {
                (Some(l), Some(r)) => {
                    if l > r {
                        continue 'upate_loop;
                    }
                }
                _ => {}
            }
        }

        let mid = update[(update.len() - 1) / 2];
        sum += mid;
        println!("valid: mid {}", mid);
    }

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_05").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut rules_portion = true;
    for line in contents.lines() {
        if line.len() == 0 {
            rules_portion = false;
        } else if rules_portion {
            let mut iter = line.split('|');
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            rules.push((l, r));
        } else {
            let update = line
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            updates.push(update);
        }
    }

    let mut incorrect = Vec::new();

    'upate_loop: for update in updates.into_iter() {
        for rule in rules.iter() {
            let l_pos = update.iter().position(|&val| val == rule.0);
            let r_pos = update.iter().position(|&val| val == rule.1);

            match (l_pos, r_pos) {
                (Some(l), Some(r)) => {
                    if l > r {
                        incorrect.push(update);
                        continue 'upate_loop;
                    }
                }
                _ => {}
            }
        }
    }

    let mut sum = 0;

    for mut update in incorrect.into_iter() {
        let relevant = rules
            .iter()
            .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
            .cloned()
            .collect::<Vec<(usize, usize)>>();

        let mut graph = HashMap::new();
        let mut in_degree = HashMap::new();

        // Initialisierung des Graphen und der In-Degree-Werte
        for &(left, right) in &relevant {
            graph.entry(left).or_insert_with(HashSet::new).insert(right);
            in_degree.entry(left).or_insert(0);
            *in_degree.entry(right).or_insert(0) += 1;
        }

        // Knoten mit In-Degree 0 in die Warteschlange einfügen
        let mut queue = VecDeque::new();
        for (&node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node);
            }
        }

        // Topologische Sortierung
        let mut sorted_rules = Vec::new();
        while let Some(node) = queue.pop_front() {
            sorted_rules.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        // Überprüfen, ob alle Knoten verarbeitet wurden (zyklischer Graph)
        if sorted_rules.len() != in_degree.len() {
            panic!("Zyklische Abhängigkeiten gefunden!");
        }

        update.sort_by_key(|&v| sorted_rules.iter().position(|&x| x == v).unwrap_or(usize::MAX));
        sum += update[(update.len() - 1) / 2];
    }

    println!("{}", sum);
}
