#![feature(vec_remove_item)]

#[macro_use]
extern crate text_io;

fn vertex_degree_count_check(mat: &Vec<Vec<i32>>) -> bool {
    let mut n_odd = 0;

    for i in mat.iter() {
        if i.len() % 2 != 0 {
            n_odd += 1;
        }

        if n_odd > 2 {
            return false;
        }
    }

    true
}

fn conectivity_test(mat: &Vec<Vec<i32>>, a: i32) -> bool {
    let mut visited: Vec<bool> = vec![false; mat.len()];
    dfs(mat, &mut visited, a);

    return all_visited(&visited);
}

fn all_visited(visited: &Vec<bool>) -> bool {
    for i in visited.iter() {
        if !i {
            return false;
        }
    }

    true
}

fn qntd_reachable_from(mat: &Vec<Vec<i32>>, a: i32) -> i32 {
    let mut visited: Vec<bool> = vec![false; mat.len()];
    dfs(mat, &mut visited, a);

    return visited.iter().map(|a| if *a { 1 } else { 0 }).sum();
}

fn dfs(mat: &Vec<Vec<i32>>, visited: &mut Vec<bool>, a: i32) {
    if all_visited(visited) {
        return;
    }

    for b in mat[a as usize].iter() {
        if !visited[*b as usize] {
            visited[*b as usize] = true;
            dfs(mat, visited, *b);
        }
    }
}

fn fleury(mat: &mut Vec<Vec<i32>>, a: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    fleury_(mat, a, &mut result);

    result
}

fn valid_edge(mat: &mut Vec<Vec<i32>>, a: i32, b: i32) -> bool {
    if mat[a as usize].len() == 1 {
        return true;
    } else {
        let q_before = qntd_reachable_from(mat, a);
        remove_edge_pair(mat, a, b);
        let q_after = qntd_reachable_from(mat, a);
        add_edge_pair(mat, a, b);

        return if q_before > q_after { false } else { true };
    }
}

fn add_edge_pair(mat: &mut Vec<Vec<i32>>, a: i32, b: i32) {
    mat[a as usize].push(b);
    mat[b as usize].push(a);
}

fn remove_edge_pair(mat: &mut Vec<Vec<i32>>, a: i32, b: i32) {
    let i1: usize = mat[a as usize].iter().position(|&r| r == b).unwrap();
    let i2: usize = mat[b as usize].iter().position(|&r| r == a).unwrap();

    mat[a as usize].remove(i1);
    mat[b as usize].remove(i2);
}

fn fleury_(mat: &mut Vec<Vec<i32>>, a: i32, result: &mut Vec<i32>) {
    let mut pivot = a;
    result.push(pivot);

    loop {
        for i in 0..mat[pivot as usize].len() {
            if mat[pivot as usize].len() > 1 {
                let i1 = mat[i as usize].iter().position(|&r| r == 0).unwrap();

                //mat[i as usize].remove(pivot);
                //mat[pivot as usize].remove(i);
            }
        }
        break;
    }
}

fn main() {
    let n_vertex: i32 = read!();
    let n_edge: i32 = read!();

    let mut mat: Vec<Vec<i32>> = vec![];

    for _ in 0..n_vertex {
        mat.push(vec![]);
    }

    for _ in 0..n_edge {
        let a: i32 = read!();
        let b: i32 = read!();
        mat[a as usize].push(b);
        mat[b as usize].push(a);
    }

    for (k, i) in mat.iter().enumerate() {
        print!("{}: ", k);
        for j in i.iter() {
            print!("{} ", j)
        }
        println!();
    }

    println!(
        "Vertex count is {}",
        if vertex_degree_count_check(&mat) {
            "Ok"
        } else {
            "not Ok"
        }
    );

    println!(
        "Graph is {}",
        if conectivity_test(&mat, n_vertex - 1) {
            "Connected"
        } else {
            "not Connected"
        }
    );
}
