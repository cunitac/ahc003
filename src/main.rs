use {
    consts::{GRID_SIZE, N_QUERY, TIME_LIMIT},
    dijkstra::dijkstra,
    judge::{Judge, Query, StdioJudge},
    step_fitting::StepFitting,
    timer::Timer,
    utility::{Edge, Node},
};

fn main() {
    let timer = Timer::new(TIME_LIMIT);
    solve(&mut StdioJudge);
    eprintln!("{}", timer.elapsed());
}

fn solve(judge: &mut impl Judge) {
    let timer = Timer::new(TIME_LIMIT);

    let mut step_h = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];
    let mut step_v = vec![StepFitting::new(GRID_SIZE - 1); GRID_SIZE];

    let mut history = vec![];

    for turn in 0..N_QUERY {
        let Query { start, goal } = judge.next_query();

        let len_h = step_h.iter().map(StepFitting::vals).collect::<Vec<_>>();
        let len_v = step_v.iter().map(StepFitting::vals).collect::<Vec<_>>();
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

        let n_recalc = if timer.elapsed() > 0.9 {
            1
        } else if turn % 10 == 0 {
            step_h.iter_mut().for_each(StepFitting::clear);
            step_v.iter_mut().for_each(StepFitting::clear);
            50
        } else {
            3
        };

        for _ in 0..n_recalc {
            let len_h = step_h.iter().map(StepFitting::vals).collect::<Vec<_>>();
            let len_v = step_v.iter().map(StepFitting::vals).collect::<Vec<_>>();
            let edge_val = |edge: Edge| match edge {
                Edge::H(i, j) => len_h[i][j],
                Edge::V(i, j) => len_v[j][i],
            };
            step_h.iter_mut().for_each(StepFitting::clear);
            step_v.iter_mut().for_each(StepFitting::clear);

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
}

pub mod consts;
pub mod dijkstra;
pub mod judge;
pub mod step_fitting;
pub mod utility;

#[cfg(test)]
mod tests;
