
pub fn solve(cost_matrix: &Vec<Vec<f64>>) -> f64 {
    /* 貪欲法により、巡回セールスマン問題を解く */    

    let mut passed = vec![false; cost_matrix.len()];
    let mut v = 0;
    passed[v as usize] = true;
    let mut answer = 0.0;
    for _ in 0..(cost_matrix.len() - 1) {
        let mut min_cost = f64::INFINITY;
        let mut min_v = -1;
        for j in 0..cost_matrix.len() {
            if passed[j as usize] {
                continue;
            }

            if min_cost > cost_matrix[v as usize][j] {
                min_cost = cost_matrix[v as usize][j];
                min_v = j as i32;
            }
        }

        passed[min_v as usize] = true;
        answer += min_cost;
        v = min_v;
    }
    answer += cost_matrix[v as usize][0];
    return answer
}