extern crate input;
extern crate timer;

use {
    crate::{
        consts::{GRID_SIZE, N_QUERY, TIME_LIMIT},
        judge::{self, Judge},
        solve,
    },
    input::*,
    std::fs::File,
    timer::Timer,
};

#[test]
fn test() {
    const N_TESTCASE: usize = 1000;

    let mut score_sum = 0.0;

    eprintln!("Score average:");

    for i in 0..N_TESTCASE {
        let file_name = format!("{:04}.txt", i);
        let mut source = Source::new(File::open(format!("tools/in/{}", file_name)).unwrap());
        let mut test = read!(from source, TestJudge);
        let timer = Timer::new(TIME_LIMIT);

        solve(&mut test);

        // assert!(!timer.is_over());
        dbg!(timer.elapsed());

        let mut out = File::create(format!("tools/out/{}", file_name)).unwrap();
        for path in &test.paths {
            use std::io::Write;
            writeln!(&mut out, "{}", path).unwrap();
        }

        score_sum += test.score() as f64;

        let showed_score = score_sum / (i as f64 + 1.0);
        eprintln!("\t{:.0}\taverage of {:4}", showed_score, i + 1);
    }
    eprintln!();
}

impl Judge for TestJudge {
    fn next_query(&mut self) -> judge::Query {
        self.queries[self.k].into()
    }
    /// ここで `k` をインクリメント
    fn path_length(&mut self, path: &str) -> u32 {
        let mut pos = self.queries[self.k].start;
        let mut len = 0;
        let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];
        for c in path.chars() {
            assert!(!visited[pos.0][pos.1],);
            visited[pos.0][pos.1] = true;
            match c {
                'U' => {
                    assert!(pos.0 > 0);
                    pos.0 -= 1;
                    len += self.len_v[pos.0][pos.1];
                }
                'L' => {
                    assert!(pos.1 > 0);
                    pos.1 -= 1;
                    len += self.len_h[pos.0][pos.1];
                }
                'D' => {
                    assert!(pos.0 + 1 < GRID_SIZE);
                    len += self.len_v[pos.0][pos.1];
                    pos.0 += 1;
                }
                'R' => {
                    assert!(pos.1 + 1 < GRID_SIZE);
                    len += self.len_h[pos.0][pos.1];
                    pos.1 += 1;
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(pos, self.queries[self.k].goal);
        self.path_lengths.push(len);
        self.paths.push(path.to_string());
        let len_with_error = (len as f64 * self.queries[self.k].error).round() as u32;
        self.k += 1;
        len_with_error
    }
}

impl TestJudge {
    fn score(&self) -> u64 {
        assert_eq!(self.k, N_QUERY);
        assert_eq!(self.paths.len(), N_QUERY);
        assert_eq!(self.path_lengths.len(), N_QUERY);

        let raw_score = (0..N_QUERY)
            .rev()
            .scan(1.0, |weight, k| {
                let a = self.queries[k].shortest as f64;
                let b = self.path_lengths[k] as f64;
                let score = *weight * (a / b);
                *weight *= 0.998;
                Some(score)
            })
            .sum::<f64>();

        (raw_score * 2_312_311.0).round() as u64
    }
}

#[derive(Clone, Debug)]
pub struct TestJudge {
    k: usize,
    len_h: Vec<Vec<u32>>,
    len_v: Vec<Vec<u32>>,
    queries: Vec<Query>,
    path_lengths: Vec<u32>,
    paths: Vec<String>,
}
#[derive(Clone, Copy, Debug)]
struct Query {
    start: (usize, usize),
    goal: (usize, usize),
    shortest: u32,
    error: f64,
}

impl FromSource for TestJudge {
    type Output = TestJudge;
    fn from_source<R: Read>(mut source: &mut Source<R>) -> Option<TestJudge> {
        macro_rules! read {
            ($($arg:tt)*) => { try_read!(from source, $($arg)*)? };
        }
        Some(TestJudge {
            k: 0,
            len_h: read!([[u32; GRID_SIZE - 1]; GRID_SIZE]),
            len_v: read!([[u32; GRID_SIZE]; GRID_SIZE - 1]),
            queries: read!([Query; N_QUERY]),
            path_lengths: Vec::with_capacity(N_QUERY),
            paths: Vec::with_capacity(N_QUERY),
        })
    }
}

impl FromSource for Query {
    type Output = Query;
    fn from_source<R: Read>(mut source: &mut Source<R>) -> Option<Query> {
        macro_rules! read {
            ($($arg:tt)*) => { try_read!(from source, $($arg)*)? };
        }
        Some(Query {
            start: read!(usize, usize),
            goal: read!(usize, usize),
            shortest: read!(u32),
            error: read!(f64),
        })
    }
}

impl From<Query> for judge::Query {
    fn from(source: Query) -> judge::Query {
        judge::Query {
            start: source.start,
            goal: source.goal,
        }
    }
}
