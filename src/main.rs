#[macro_use]
extern crate input as _;

const N_QUERY: usize = 1000;

fn main() {
    for _ in 0..N_QUERY {
        input!(start: (usize, usize), goal: (usize, usize));

        print!(
            "{}",
            if goal.0 < start.0 {
                "U".repeat(start.0 - goal.0)
            } else {
                "D".repeat(goal.0 - start.0)
            }
        );
        println!(
            "{}",
            if goal.1 < start.1 {
                "L".repeat(start.1 - goal.1)
            } else {
                "R".repeat(goal.1 - start.1)
            }
        );

        input!(_path_len: u64);
    }
}
