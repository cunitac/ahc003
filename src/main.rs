use {
    consts::{GRID_SIZE, N_EDGE, N_QUERY},
    dijkstra::dijkstra,
    judge::{Judge, Query, StdioJudge},
    utility::{Edge, Node, Samples},
};

fn main() {
    solve(&mut StdioJudge)
}

fn solve(judge: &mut impl Judge) {
    let mut edge_len = vec![Samples::new(); N_EDGE];
    let mut road_h = vec![vec![Samples::new(); 2]; GRID_SIZE];
    let mut road_v = vec![vec![Samples::new(); 2]; GRID_SIZE];

    for _ in 0..N_QUERY {
        let Query { start, goal } = judge.next_query();

        let road_val = |edge: Edge| match edge {
            Edge::H(i, j) => road_h[i][j / 15].val(),
            Edge::V(i, j) => road_v[j][i / 15].val(),
        };
        const ROAD_VAL_WEIGHT: f64 = 2.0;
        let edge_val = |edge: Edge| edge_len[edge].val() + ROAD_VAL_WEIGHT * road_val(edge);
        let adj = |node: Node| {
            node.out_nodes()
                .map(move |adj| (adj, edge_val(Edge::between(node, adj).unwrap().0)))
        };

        let (path, _nodes, edges) = dijkstra(start.into(), goal.into(), adj);

        let length = judge.path_length(&path) as f64;
        let length_e_sum = edges.iter().map(|&edge| edge_len[edge].val()).sum::<f64>();

        for &edge in &edges {
            let len = edge_len[edge].val();
            let sample = length * len / length_e_sum;
            edge_len[edge].add(sample);
            match edge {
                Edge::H(i, j) => road_h[i][j / 15].add(sample),
                Edge::V(i, j) => road_v[j][i / 15].add(sample),
            }
        }
    }
}

mod consts;
mod dijkstra;
mod judge;
mod utility;

#[cfg(test)]
mod tests;
