use {
    consts::N_QUERY,
    judge::{Judge, Query, StdioJudge},
};

fn main() {
    solve(&mut StdioJudge)
}

fn solve(judge: &mut impl Judge) {
    for _ in 0..N_QUERY {
        let Query { start, goal } = judge.next_query();

        let mut path = String::new();

        path += &if goal.0 < start.0 {
            "U".repeat(start.0 - goal.0)
        } else {
            "D".repeat(goal.0 - start.0)
        };
        path += &if goal.1 < start.1 {
            "L".repeat(start.1 - goal.1)
        } else {
            "R".repeat(goal.1 - start.1)
        };

        judge.path_length(&path);
    }
}

mod consts;
mod judge;

#[cfg(test)]
mod tests;
