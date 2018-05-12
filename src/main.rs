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
