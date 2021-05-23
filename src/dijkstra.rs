use {
    crate::{
        consts::N_NODE,
        utility::{Edge, Node},
    },
    std::collections::BinaryHeap,
};

#[allow(clippy::float_cmp)]
pub fn dijkstra<Adj, It>(start: Node, goal: Node, adj: Adj) -> (String, Vec<Node>, Vec<Edge>)
where
    Adj: Fn(Node) -> It,
    It: IntoIterator<Item = (Node, f64)>,
{
    let mut dist = vec![None::<f64>; N_NODE];
    let mut prev = vec![None; N_NODE];
    let mut heap = BinaryHeap::new();

    dist[start] = Some(0.0);
    heap.push(KeyValue(0.0, start));

    while let Some(KeyValue(dist_v, v)) = heap.pop() {
        if dist[v].unwrap() != dist_v {
            continue;
        }
        if goal == v {
            break;
        }
        for (u, len) in adj(v) {
            let dist_u_new = dist_v + len;
            if let Some(dist_u) = dist[u].as_mut() {
                if *dist_u > dist_u_new {
                    *dist_u = dist_u_new;
                    prev[u] = Some(v);
                    heap.push(KeyValue(dist_u_new, u));
                }
            } else {
                dist[u] = Some(dist_u_new);
                prev[u] = Some(v);
                heap.push(KeyValue(dist_u_new, u))
            }
        }
    }

    let mut nodes = vec![goal];
    let mut edges = vec![];
    let mut path = vec![];

    let mut pos = goal;
    while let Some(prev) = prev[pos] {
        nodes.push(prev);
        let (edge, direction) = Edge::between(prev, pos).unwrap();
        edges.push(edge);
        path.push(direction);
        pos = prev;
    }

    nodes.reverse();
    edges.reverse();
    let path = path.into_iter().rev().collect();

    (path, nodes, edges)
}

#[derive(PartialEq)]
struct KeyValue(f64, Node);

impl Eq for KeyValue {}
impl PartialOrd for KeyValue {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        rhs.0.partial_cmp(&self.0)
    }
}
impl Ord for KeyValue {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.0.partial_cmp(&self.0).unwrap()
    }
}
