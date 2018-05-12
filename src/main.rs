#[macro_use]
extern crate text_io;

fn vertex_degree_count_check(mat: Vec<Vec<i32>>) -> bool {
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
        if vertex_degree_count_check(mat) {
            "Ok"
        } else {
            "not Ok"
        }
    );
}
