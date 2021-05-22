extern crate input;

use input::*;

pub trait Judge {
    /// 次のクエリを返す。
    ///
    /// `Judge` の作成直後か、`path_length` の直後に呼ばれなければならない。
    fn next_query(&mut self) -> Query;
    /// 直前のクエリに対し、`path` を解答し、`path` によって表されるパスの長さを得る。
    ///
    /// `next_length` の直後に呼ばれなければならない。
    fn path_length(&mut self, path: &str) -> u32;
}
pub struct Query {
    pub start: (usize, usize),
    pub goal: (usize, usize),
}

pub struct StdioJudge;
impl Judge for StdioJudge {
    fn next_query(&mut self) -> Query {
        input!(start: (usize, usize), goal: (usize, usize));
        Query { start, goal }
    }
    fn path_length(&mut self, path: &str) -> u32 {
        println!("{}", path);
        read!(u32)
    }
}
