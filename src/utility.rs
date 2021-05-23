extern crate grid;

use {
    crate::consts::{GRID_SIZE, N_EDGE},
    std::ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Debug)]
pub struct Samples {
    pub n: usize,
    pub sum: f64,
}

impl Samples {
    pub fn new() -> Self {
        Samples { n: 0, sum: 0.0 }
    }
    pub fn checked_val(&self) -> Option<f64> {
        if self.n == 0 {
            None
        } else {
            Some(self.sum / self.n as f64)
        }
    }
    pub fn val(&self) -> f64 {
        self.checked_val().unwrap_or(4000.0)
    }
    pub fn add(&mut self, sample: f64) {
        self.n += 1;
        self.sum += sample
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Edge {
    H(usize, usize),
    V(usize, usize),
}

impl Edge {
    /// (Edge, a->b direction)
    pub fn between(a: Node, b: Node) -> Option<(Edge, char)> {
        if a.0 == b.0 {
            if a.1 + 1 == b.1 {
                return Some((Edge::H(a.0, a.1), 'R'));
            } else if a.1 == b.1 + 1 {
                return Some((Edge::H(b.0, b.1), 'L'));
            }
        } else if a.1 == b.1 {
            if a.0 + 1 == b.0 {
                return Some((Edge::V(a.0, a.1), 'D'));
            } else if a.0 == b.0 + 1 {
                return Some((Edge::V(b.0, b.1), 'U'));
            }
        }

        None
    }
}

impl From<Edge> for usize {
    fn from(edge: Edge) -> usize {
        match edge {
            Edge::H(i, j) => i * (GRID_SIZE - 1) + j,
            Edge::V(i, j) => N_EDGE / 2 + i * GRID_SIZE + j,
        }
    }
}

impl From<usize> for Edge {
    fn from(index: usize) -> Edge {
        if index < N_EDGE / 2 {
            Edge::H(index / (GRID_SIZE - 1), index % (GRID_SIZE - 1))
        } else {
            let index = index - N_EDGE / 2;
            Edge::V(index / GRID_SIZE, index % GRID_SIZE)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Node(usize, usize);

impl Node {
    pub fn out_edges(self) -> impl Iterator<Item = Edge> {
        grid::adj4(self.into(), (GRID_SIZE, GRID_SIZE))
            .map(move |adj| Edge::between(self, adj.into()).unwrap().0)
    }
    pub fn out_nodes(self) -> impl Iterator<Item = Node> {
        grid::adj4(self.into(), (GRID_SIZE, GRID_SIZE)).map(Node::from)
    }
}

impl From<usize> for Node {
    fn from(index: usize) -> Node {
        Node(index / GRID_SIZE, index % GRID_SIZE)
    }
}

impl From<(usize, usize)> for Node {
    fn from((i, j): (usize, usize)) -> Node {
        Node(i, j)
    }
}

impl From<Node> for usize {
    fn from(node: Node) -> usize {
        node.0 * GRID_SIZE + node.1
    }
}

impl From<Node> for (usize, usize) {
    fn from(node: Node) -> (usize, usize) {
        (node.0, node.1)
    }
}

impl<T> Index<Edge> for Vec<T> {
    type Output = T;
    fn index(&self, index: Edge) -> &T {
        &self[usize::from(index)]
    }
}

impl<T> Index<Node> for Vec<T> {
    type Output = T;
    fn index(&self, index: Node) -> &T {
        &self[usize::from(index)]
    }
}

impl<T> IndexMut<Edge> for Vec<T> {
    fn index_mut(&mut self, index: Edge) -> &mut T {
        &mut self[usize::from(index)]
    }
}

impl<T> IndexMut<Node> for Vec<T> {
    fn index_mut(&mut self, index: Node) -> &mut T {
        &mut self[usize::from(index)]
    }
}
