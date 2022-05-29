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
    // 頂点数N^2 - 1の木を見つける
    let mut tile_count = {
        let mut count = vec![0; 16];
        for row in input.tiles.iter() {
            for t in row.iter() {
                count[*t] += 1;
            }
        }
        count
    };
    let mut now_tiles = vec![vec![0; input.n]; input.n];
    let mut best_tiles = vec![vec![0; input.n]; input.n];
    let mut next_poses = vec![];
    if dfs(
        (0, 0),
        &input,
        &mut tile_count,
        &mut now_tiles,
        &mut best_tiles,
        &mut next_poses,
    ) {
        for row in now_tiles {
            for t in row.iter() {
                print!("{:2} ", t);
            }
            println!();
        }
    }
}

fn dfs(
    pos: (usize, usize),
    input: &Input,
    tile_count: &mut [i32],
    now_tiles: &mut [Vec<usize>],
    best_tiles: &mut [Vec<usize>],
    next_poses: &mut Vec<(usize, usize)>,
) -> bool {
    // 今のposに置くタイルを決める
    for tile_i in 1..16 {
        if tile_count[tile_i] == 0 {
            // このタイルはない
            continue;
        }
        let mut is_place = true;
        let mut dij = vec![];
        for (d, (di, dj)) in DIJ.iter().enumerate() {
            // L, U, R, D
            let i2 = pos.0 + *di;
            let j2 = pos.1 + *dj;
            if (tile_i >> d) & 1 == 1 {
                if i2 < input.n
                    && j2 < input.n
                    && (now_tiles[i2][j2] == 0 || ((now_tiles[i2][j2] >> (d ^ 2)) & 1 == 1))
                {
                    // 空きマスか、今置くタイルと繋がっているか
                    if now_tiles[i2][j2] == 0 {
                        dij.push((i2, j2));
                    }
                } else {
                    // このタイルは置けない
                    is_place = false;
                    break;
                }
            } else {
                // このタイルが開いていない方向が別のタイルの開いている方向なら、このタイルは置けない
                if i2 < input.n && j2 < input.n && (now_tiles[i2][j2] >> (d ^ 2)) & 1 == 1 {
                    // このタイルは置けない
                    is_place = false;
                    break;
                }
            }
        }
        if is_place {
            if dij.is_empty() {
                now_tiles[pos.0][pos.1] = tile_i;
                tile_count[tile_i] -= 1;
                if tile_count.iter().skip(1).sum::<i32>() == 0 {
                    return true;
                }
                for ni in (0..next_poses.len()).rev() {
                    if now_tiles[next_poses[ni].0][next_poses[ni].1] != 0 {
                        continue;
                    }
                    if dfs(
                        (next_poses[ni].0, next_poses[ni].1),
                        input,
                        tile_count,
                        now_tiles,
                        best_tiles,
                        next_poses,
                    ) {
                        return true;
                    }
                }
                now_tiles[pos.0][pos.1] = 0;
                tile_count[tile_i] += 1;
            } else {
                next_poses.extend_from_slice(&dij);
                for &(i2, j2) in dij.iter() {
                    now_tiles[pos.0][pos.1] = tile_i;
                    tile_count[tile_i] -= 1;
                    if dfs(
                        (i2, j2),
                        input,
                        tile_count,
                        now_tiles,
                        best_tiles,
                        next_poses,
                    ) {
                        return true;
                    }
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                }
                for _ in 0..dij.len() {
                    next_poses.pop();
                }
            }
        }
    }
    false
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
