use proconio::{input, marker::Chars};

pub type Output = Vec<char>;

pub const DIJ: [(usize, usize); 4] = [(0, !0), (!0, 0), (0, 1), (1, 0)];
pub const DIR: [char; 4] = ['L', 'U', 'R', 'D'];

pub struct Input {
    pub n: usize,
    pub t: usize,
    pub tiles: Vec<Vec<usize>>,
}
fn main() {
    let input = parse_input();
}

fn parse_input() -> Input {
    input! {
        n: usize,
        t: usize,
        tiles: [Chars; n]
    }
    let tiles = tiles
        .iter()
        .map(|ts| {
            ts.iter()
                .map(|&c| usize::from_str_radix(&c.to_string(), 16).unwrap())
                .collect()
        })
        .collect();
    Input { n, t, tiles }
}
