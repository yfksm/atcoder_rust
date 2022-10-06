use proconio::input;
use std::collections::HashMap;

fn main() {
    // stdin
    input! {
        _n: i32,
        _x: i32,
        _y: i32,
    }

    let mut _map: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for _i in 1.._n {
        input! {
            _u: i32,
            _v: i32,
        }

        _map.entry(_u).or_insert_with(|| vec![]).push(_v);
        _map.entry(_v).or_insert_with(|| vec![]).push(_u);
    }


    let mut _ans: Vec<i32> = vec![];
    let mut _visited: Vec<bool> = vec![false; 200010];
    let mut _finish: bool = false;

    dfs(_x, _y, &mut _ans, &mut _visited, &_map, &mut _finish);

    for _i in _ans {
        print!("{} ", _i);
    }
}

fn dfs(current: i32, goal: i32, mut ans: &mut Vec<i32>, mut visited: &mut Vec<bool>, map: &HashMap<i32, Vec<i32>>, mut finish: &mut bool) {
    visited[current as usize] = true;
    (*ans).push(current);

    if current == goal {
        *finish = true;
        return;
    }

    for element in &map[&current] {
        if !(*finish) && !visited[*element as usize] {
            dfs(*element, goal, &mut ans, &mut visited, &map, &mut finish);
        }
    }

    if !(*finish) {
        (*ans).pop();
    }
}

