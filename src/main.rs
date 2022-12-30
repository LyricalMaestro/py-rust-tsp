mod tsp_solver;
use tsp_solver::solve;

fn main() {
    let n = 100;
    let mut cost_matrix: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        let row = vec![i as f64;n];
        cost_matrix.push(row);
    }

    let answer = solve(&cost_matrix);
    println!("{}", answer);
}
