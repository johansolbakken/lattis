use std::collections::{HashMap, HashSet};

use crate::parser;

pub fn reaching_definitions(node: &parser::Node) -> usize {
    let mut iteration = 1;

    let mut data_points_a = HashMap::<String, HashSet<String>>::new();
    let mut data_points_b = HashMap::<String, HashSet<String>>::new();
    find_data_points(&mut data_points_a, node);
    find_data_points(&mut data_points_b, node);

    loop {
        for data_flow_equation in node.children.iter() {
            let l = &data_flow_equation.children[0];
            let name = (&l.token).as_ref().unwrap().lexeme.clone();
            let r = &data_flow_equation.children[1];
            let points = solve_points(&mut data_points_a, r);
            data_points_a.insert(name, points);
        }

        println!("Iteration {}", iteration);
        print_datapoints(&data_points_a);

        if !has_changed(&data_points_a, &data_points_b) {
            break;
        }

        data_points_b = data_points_a.clone();
        iteration += 1;
    }

    return iteration;
}

fn solve_points(
    data_points: &mut HashMap<String, HashSet<String>>,
    node: &parser::Node,
) -> HashSet<String> {
    let mut points = HashSet::new();

    // Copy other data points
    if node.node_type == parser::NodeType::DataPoint {
        let token = node.token.as_ref().unwrap();
        let name = token.lexeme.clone();
        let other_points = data_points.get(&name).unwrap();
        points.extend(other_points.iter().cloned());
    } else if node.node_type == parser::NodeType::Definition {
        let token = node.token.as_ref().unwrap();
        let name = token.lexeme.clone();
        points.insert(name);
    } else if node.node_type == parser::NodeType::SetDifference {
        let left = &node.children[0];
        let right = &node.children[1];
        let left_points = solve_points(data_points, left);
        let right_points = solve_points(data_points, right);
        points.extend(left_points.iter().cloned());
        points.retain(|x| !right_points.contains(x));
    } else if node.node_type == parser::NodeType::Union {
        let left = &node.children[0];
        let right = &node.children[1];
        let left_points = solve_points(data_points, left);
        let right_points = solve_points(data_points, right);
        points.extend(left_points.iter().cloned());
        points.extend(right_points.iter().cloned());
    } else {
        for child in &node.children {
            let child_points = solve_points(data_points, child);
            points.extend(child_points.iter().cloned());
        }
    }

    return points;
}

fn has_changed(a: &HashMap<String, HashSet<String>>, b: &HashMap<String, HashSet<String>>) -> bool {
    for (key, value) in a {
        let other_value = b.get(key).unwrap();
        if value.difference(other_value).count() > 0 {
            return true;
        }
    }
    return false;
}

fn print_datapoints(data_points: &HashMap<String, HashSet<String>>) {
    let mut keys = Vec::new();
    for (key, _) in data_points {
        keys.push(key.clone());
    }
    keys.sort_by(|a, b| {
        let mut a = a.clone();
        a.remove(0);
        let mut b = b.clone();
        b.remove(0);
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        a.cmp(&b)
    });

    for key in &keys {
        print!("{}: {{", key);
        let points = data_points.get(key).unwrap();
        let mut points = points.iter().collect::<Vec<_>>();
        points.sort_by(|a, b| {
            let mut a = (*a).clone();
            a.remove(0);
            let mut b = (*b).clone();
            b.remove(0);
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            a.cmp(&b)
        });

        for point in points {
            print!("{}, ", point);
        }
        println!("}}");
    }
}

fn find_data_points(data_points: &mut HashMap<String, HashSet<String>>, node: &parser::Node) {
    for child in &node.children {
        let l = &child.children[0];
        let name = (&l.token).as_ref().unwrap().lexeme.clone();
        data_points.insert(name, HashSet::new());
    }
}
