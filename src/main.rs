use {
    consts::{GRID_SIZE, N_QUERY},
    dijkstra::dijkstra,
    judge::{Judge, Query, StdioJudge},
    step_fitting::StepFitting,
    timer::Timer,
    utility::{Edge, Node},
};

fn main() {
    let timer = Timer::new(2.0);
    solve(&mut StdioJudge);
    eprintln!("{}", timer.elapsed());
}

fn solve(judge: &mut impl Judge) {
    let mut step_h = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];
    let mut step_v = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];

    let mut history = vec![];

    for turn in 0..N_QUERY {
        let Query { start, goal } = judge.next_query();

        let time = turn as f64 / N_QUERY as f64;
        let road_weight = if time < 0.3 {
            time * 1.55 / 0.3
        } else {
            2.0 - time * 1.5
        };
        let mut len_h = step_h
            .iter()
            .map(|step| step.vals(road_weight))
            .collect::<Vec<_>>();
        let mut len_v = step_v
            .iter()
            .map(|step| step.vals(road_weight))
            .collect::<Vec<_>>();
        let edge_val = |edge: Edge| match edge {
            Edge::H(i, j) => len_h[i][j],
            Edge::V(i, j) => len_v[j][i],
        };
        let adj = |node: Node| {
            node.out_nodes()
                .map(move |adj| (adj, edge_val(Edge::between(node, adj).unwrap().0)))
        };
        let (path, _nodes, edges) = dijkstra(start.into(), goal.into(), adj);
        let length = judge.path_length(&path) as f64;
        history.push((edges.clone(), length));

        len_h = step_h
            .iter()
            .map(|step| step.vals(road_weight))
            .collect::<Vec<_>>();
        len_v = step_v
            .iter()
            .map(|step| step.vals(road_weight))
            .collect::<Vec<_>>();
        let edge_val = |edge: Edge| match edge {
            Edge::H(i, j) => len_h[i][j],
            Edge::V(i, j) => len_v[j][i],
        };

        step_h = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];
        step_v = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];
        for (edges, length) in &history {
            let length_e_sum = edges.iter().map(|&edge| edge_val(edge)).sum::<f64>();

            for &edge in edges {
                let len = edge_val(edge);
                let sample = length * len / length_e_sum;
                match edge {
                    Edge::H(i, j) => step_h[i].add(j, sample),
                    Edge::V(i, j) => step_v[j].add(i, sample),
                }
            }
        }
    }
}

pub mod consts;
pub mod dijkstra;
pub mod judge;
pub mod step_fitting;
pub mod utility;

#[cfg(test)]
mod tests;
