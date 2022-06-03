#![allow(
    clippy::needless_range_loop,
    clippy::comparison_chain,
    clippy::collapsible_else_if,
    clippy::same_item_push
)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::{input, marker::Chars};
use std::collections::{HashMap, VecDeque};

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
    now_tiles[input.n - 1][input.n - 1] = 16;
    let mut next_poses = vec![];
    let tile_is = get_tile_is(&input);
    let mut count = 0;
    if dfs(
        (0, 0),
        &input,
        &mut tile_count,
        &mut now_tiles,
        &mut next_poses,
        &tile_is,
        &mut count,
    ) {
        eprintln!("count: {}", count);
        for row in now_tiles.iter() {
            for t in row.iter() {
                eprint!("{:2} ", t);
            }
            eprintln!();
        }
    }
    // 見つけた木となるような操作列の構築
    let out = construct(&input, &mut now_tiles);
    println!("{}", out.iter().take(input.t).join(""));
}

fn get_tile_is(input: &Input) -> Vec<Vec<Vec<usize>>> {
    let mut tile_is = vec![vec![vec![]; input.n]; input.n];
    for pi in 0..input.n {
        for pj in 0..input.n {
            let mut is = vec![];
            let mut que = VecDeque::new();
            let mut visited = vec![vec![false; input.n]; input.n];
            let mut used_tile_i = vec![false; 16];
            visited[pi][pj] = true;
            que.push_back((pi, pj));
            used_tile_i[0] = true;
            while !que.is_empty() && used_tile_i.iter().any(|b| !*b) {
                let v = que.pop_front().unwrap();
                for (di, dj) in DIJ.iter() {
                    let ni = v.0 + *di;
                    let nj = v.1 + *dj;
                    if input.n <= ni || input.n <= nj {
                        continue;
                    }
                    if visited[ni][nj] {
                        continue;
                    }
                    let tile_i = if input.tiles[ni][nj] != 16 {
                        input.tiles[ni][nj]
                    } else {
                        0
                    };
                    if !used_tile_i[tile_i] {
                        is.push(tile_i);
                        used_tile_i[tile_i] = true;
                    }
                    visited[ni][nj] = true;
                    que.push_back((ni, nj));
                }
            }
            tile_is[pi][pj] = is;
        }
    }
    tile_is
}

fn get_now(
    tar: (usize, usize),
    fix: &[Vec<bool>],
    input: &Input,
    tiles: &[Vec<usize>],
    tree_tiles: &[Vec<usize>],
) -> (usize, usize) {
    // TODO: ここをbfsにする
    // fix[i][j] or visited[i][j]とかにすればできるだろ

    let mut que = VecDeque::new();
    let mut visited = vec![vec![false; input.n]; input.n];
    visited[tar.0][tar.1] = true;
    que.push_back(tar);
    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        if !fix[v.0][v.1] && tree_tiles[tar.0][tar.1] == tiles[v.0][v.1] {
            return v;
        }
        for (di, dj) in DIJ.iter() {
            let ni = v.0 + *di;
            let nj = v.1 + *dj;
            if input.n <= ni || input.n <= nj {
                continue;
            }
            if fix[ni][nj] || visited[ni][nj] {
                continue;
            }
            visited[ni][nj] = true;
            que.push_back((ni, nj));
        }
    }
    // for i in start..input.n {
    //     for j in start..input.n {
    //         if fix[i][j] {
    //             continue;
    //         }
    //         if tree_tiles[tar.0][tar.1] == tiles[i][j] {
    //             return (i, j);
    //         }
    //     }
    // }
    unreachable!("not found: now:(");
}

fn construct(input: &Input, tree_tiles: &mut [Vec<usize>]) -> Output {
    // inputのタイルと木のタイルで番号付けを行う
    // スライディングパズルを解く
    let mut tiles = input.tiles.clone();
    for row in tiles.iter_mut() {
        for t in row.iter_mut() {
            if *t == 0 {
                *t = 16;
            }
        }
    }
    let mut fix = vec![vec![false; input.n]; input.n];

    let mut out = vec![];
    for i in 0..input.n {
        if i == input.n - 3 {
            break;
        }
        for j in i..input.n {
            // tiles[i][j]をtree_tiles[i][j]にする
            if i < input.n - 2 && j < input.n - 2 {
                let now = get_now((i, j), &fix, input, &tiles, tree_tiles);
                let out_i = slide((i, j), now, input, &mut tiles);
                fix[i][j] = true;
                for oi in out_i {
                    out.push(oi);
                }
            } else {
                // 2個入れる
                if j == input.n - 1 {
                    continue;
                }
                let out_i = slide2((i, j + 1), (i, j), &mut fix, input, &mut tiles, tree_tiles);
                fix[i][j] = true;
                fix[i][j + 1] = true;
                for oi in out_i {
                    out.push(oi);
                }
            }
        }
        for i2 in i + 1..input.n {
            // tiles[i2][i]
            if i2 < input.n - 2 && i < input.n - 2 {
                let now = get_now((i2, i), &fix, input, &tiles, tree_tiles);
                let out_i = slide((i2, i), now, input, &mut tiles);
                fix[i2][i] = true;
                for oi in out_i {
                    out.push(oi);
                }
            } else {
                // 2個入れる
                if i2 == input.n - 1 {
                    continue;
                }
                let out_i = slide2(
                    (i2 + 1, i),
                    (i2, i),
                    &mut fix,
                    input,
                    &mut tiles,
                    tree_tiles,
                );
                fix[i2][i] = true;
                fix[i2 + 1][i] = true;
                for oi in out_i {
                    out.push(oi);
                }
            }
        }
    }
    // 3*3を完成させる
    let out_i = slide3x3(input, &mut tiles, tree_tiles);
    for oi in out_i {
        out.push(oi);
    }
    // 3x3でtilesを動かしてないのでここで出力しても一見そろってない
    // が、goalしていたらoutは揃っている
    out
}

fn slide3x3(input: &Input, tiles: &mut [Vec<usize>], tree_tiles: &mut [Vec<usize>]) -> Output {
    let mut out = vec![];

    let mut tiles3x3 = vec![];
    for i in 0..3 {
        for j in 0..3 {
            if tiles[i + input.n - 3][j + input.n - 3] == 16 {
                tiles3x3.push(0);
            } else {
                tiles3x3.push(tiles[i + input.n - 3][j + input.n - 3]);
            }
        }
    }

    let mut goal = vec![];
    for i in 0..3 {
        for j in 0..3 {
            if tree_tiles[i + input.n - 3][j + input.n - 3] == 16 {
                goal.push(0);
            } else {
                goal.push(tree_tiles[i + input.n - 3][j + input.n - 3]);
            }
        }
    }
    // 盤面列挙
    let mut ord = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut vs = vec![];
    loop {
        let mut v = 0;
        let mut n = 1;
        for o in ord.iter() {
            v += tiles3x3[*o] * n;
            n *= 16;
        }
        vs.push(v);
        if !ord.next_permutation() {
            break;
        }
    }
    vs.sort_unstable();
    vs.dedup();
    let map = vs
        .into_iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect::<HashMap<usize, usize>>();
    let tiles2num = |tiles3x3: &[usize]| -> usize {
        let mut v = 0;
        let mut n = 1;
        for t in tiles3x3.iter() {
            v += *t * n;
            n *= 16;
        }
        v
    };
    let num2tiles = |num: usize, n: usize| -> Vec<usize> {
        let mut v = num;
        let mut n = n;
        let mut vs = vec![];
        while v > 0 {
            n /= 16;
            vs.push(v / n);
            v %= n;
        }
        if vs.len() < 9 {
            vs.push(v);
        }
        vs.reverse();
        vs
    };

    let get_empty = |vs: &[usize]| -> (usize, usize) {
        for i in 0..3 {
            for j in 0..3 {
                if 0 == vs[i * 3 + j] {
                    return (i, j);
                }
            }
        }
        unreachable!();
    };

    // bfs
    let mut que = std::collections::VecDeque::new();
    let mut dist = vec![-1; map.len()];
    let mut prev = vec![4; map.len()];
    let s = tiles2num(&tiles3x3);
    let g = tiles2num(&goal);
    if s == g {
        return out;
    }
    const N: usize = 68719476736;
    dist[map[&s]] = 0;
    que.push_back(s);
    'lp: while !que.is_empty() {
        let v = que.pop_front().unwrap();
        let vs = num2tiles(v, N);
        let empty = get_empty(&vs);
        for (i, &(di, dj)) in DIJ.iter().enumerate() {
            let ni = empty.0 + di;
            let nj = empty.1 + dj;
            if 3 <= ni || 3 <= nj {
                continue;
            }
            let mut new_vs = vs.clone();
            new_vs.swap(empty.0 * 3 + empty.1, ni * 3 + nj);
            let nv = tiles2num(&new_vs);
            if dist[map[&nv]] != -1 {
                continue;
            }
            dist[map[&nv]] = dist[map[&v]] + 1;
            prev[map[&nv]] = i;
            que.push_back(nv);
            if nv == g {
                break 'lp;
            }
        }
    }
    // 経路復元
    let mut empty = get_empty(&goal);
    let mut v = g;
    let mut v_tiles = goal;
    while v != s {
        let i = prev[map[&v]];
        out.push(DIR[i]);
        let ni = empty.0 + DIJ[i ^ 2].0;
        let nj = empty.1 + DIJ[i ^ 2].1;
        v_tiles.swap(empty.0 * 3 + empty.1, ni * 3 + nj);
        v = tiles2num(&v_tiles);
        empty = (ni, nj);
    }
    out.reverse();
    out
}

fn slide2(
    tar_a: (usize, usize),
    tar_b: (usize, usize),
    fix: &mut [Vec<bool>],
    input: &Input,
    tiles: &mut [Vec<usize>],
    tree_tiles: &mut [Vec<usize>],
) -> Vec<char> {
    let mut out: Output = vec![];
    let start = tar_b.0.min(tar_b.1);
    let mut a_now = get_now(tar_a, fix, input, tiles, tree_tiles);
    let mut b_now = get_now(tar_b, fix, input, tiles, tree_tiles);

    let mut empty = || -> (usize, usize) {
        for i in start..input.n {
            for j in start..input.n {
                if (i < tar_b.0 && j == tar_b.1) || (i == tar_b.0 && j < tar_b.1) {
                    continue;
                }
                if 16 == tiles[i][j] {
                    return (i, j);
                }
            }
        }
        (input.n, input.n)
    }();
    // tar_a,tar_bとそれぞれの1マス隣の4マスからbを排除しなくていいパターンを列挙 6パターン
    if a_now == tar_a && b_now == tar_b {
        return out;
    }
    if tar_a.0 <= tar_a.1 {
        // 上三角を見る
        if b_now == tar_b && empty == tar_a && a_now == (tar_a.0 + 1, tar_a.1) {
            // 空きマスを1つ下げればOK
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            return out;
        }
        if a_now == tar_a && empty == tar_b && b_now == (tar_b.0 + 1, tar_b.1) {
            // 空きマスを1つ下げればOK
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            return out;
        }
        if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // a.
            // b.
            // 一回回す
            // a_nowとb_nowは想定と違う可能性があるためセットする
            a_now = tar_b;
            b_now = (tar_b.0 + 1, tar_b.1);
            // 空きマスを持ってくる
            // b_now.0 + 1の行に持ってくる
            if empty.0 <= b_now.0 + 1 {
                for _ in 0..(b_now.0 + 1 - empty.0) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            } else {
                for _ in 0..(empty.0 - b_now.0 - 1) {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
            }
            // 一番右の列に持って行く
            for _ in 0..(input.n - 1 - empty.1) {
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
            }
            // aの隣に持って行く
            for _ in 0..(empty.0 - a_now.0) {
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
            }
            // 1回回す
            out.push('L');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
            tiles[empty.0][empty.1 - 1] = 16;
            empty.1 -= 1;
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            return out;
        }
        if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // ..
            // ab
            // a_nowとb_nowは想定と違う可能性があるためセットする
            a_now = (tar_b.0 + 1, tar_b.1);
            // b_now = (tar_b.0 + 1, tar_b.1 + 1);
            // 空きマスの位置によってさらに場合分け
            if empty.0 < a_now.0 {
                // 2回回す
                if empty.1 == input.n - 1 {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                return out;
            } else {
                // aをtar_bにスライドさせて、その後bをtar_bにスライドさせると揃う
                // 注意 a_nowを再計算しないようにする b_nowは？
                // aを動かさない場所にあるはず？だから再計算しても良さそう
                // tar_aにあるbを触りたくないのでfixしておく
                let out1 = slide(tar_b, a_now, input, tiles);
                for oi in out1 {
                    out.push(oi);
                }
                fix[tar_b.0][tar_b.1] = true;
                fix[tar_a.0][tar_a.1] = true;
                let b_now = get_now(tar_b, fix, input, tiles, tree_tiles);
                fix[tar_b.0][tar_b.1] = false;
                fix[tar_a.0][tar_a.1] = false;
                let out2 = slide(tar_b, b_now, input, tiles);
                for oi in out2 {
                    out.push(oi);
                }
                return out;
            }
        }
        if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // .b
            // .a
            // a_nowとb_nowは想定と違う可能性があるためセットする
            // a_now = (tar_b.0 + 1, tar_b.1 + 1);
            b_now = (tar_b.0, tar_b.1 + 1);
            // 1回回す ただし上のパターンまでで回していたのと逆回り
            // まずinput.n - 2の列に持って行く
            if empty.1 < input.n - 2 {
                for _ in 0..(input.n - 2 - empty.1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            } else {
                for _ in 0..(empty.1 - input.n - 2) {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
            }
            // bの隣に持って行く
            for _ in 0..(empty.0 - b_now.0) {
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
            }
            // 回す
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            return out;
        }
    } else {
        // 下三角を見る
        if b_now == tar_b && empty == tar_a && a_now == (tar_a.0, tar_a.1 + 1) {
            // 空きマスを右にずらせばOK
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            return out;
        }
        if a_now == tar_a && empty == tar_b && b_now == (tar_b.0, tar_b.1 + 1) {
            // 空きマスを1つ下げればOK
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            return out;
        }
        if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // ab
            // ..
            // 一回回す
            // a_nowとb_nowは想定と違う可能性があるためセットする
            a_now = tar_b;
            b_now = (tar_b.0, tar_b.1 + 1);
            // 空きマスを持ってくる
            // b_now.1 + 1の列に持ってくる
            if empty.1 <= b_now.1 + 1 {
                for _ in 0..(b_now.1 + 1 - empty.1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            } else {
                for _ in 0..(empty.1 - b_now.1 - 1) {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
            }
            // 一番下の行に持って行く
            for _ in 0..(input.n - 1 - empty.0) {
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
            }
            // aの隣に持って行く
            for _ in 0..(empty.1 - a_now.1) {
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
            }
            // 1回回す
            out.push('U');
            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
            tiles[empty.0 - 1][empty.1] = 16;
            empty.0 -= 1;
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            return out;
        }
        if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // .a
            // .b
            // a_nowとb_nowは想定と違う可能性があるためセットする
            a_now = (tar_b.0, tar_b.1 + 1);
            // b_now = (tar_b.0 + 1, tar_b.1 + 1);
            // 空きマスの位置によってさらに場合分け
            if empty.1 < a_now.1 {
                // 2回回す
                if empty.0 == input.n - 1 {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                return out;
            } else {
                // aをtar_bにスライドさせて、その後bをtar_bにスライドさせると揃う
                // 注意 a_nowを再計算しないようにする b_nowは？
                // aを動かさない場所にあるはず？だから再計算しても良さそう
                // tar_aにあるbを触りたくないのでfixしておく
                let out1 = slide(tar_b, a_now, input, tiles);
                for oi in out1 {
                    out.push(oi);
                }
                fix[tar_b.0][tar_b.1] = true;
                fix[tar_a.0][tar_a.1] = true;
                let b_now = get_now(tar_b, fix, input, tiles, tree_tiles);
                fix[tar_b.0][tar_b.1] = false;
                fix[tar_a.0][tar_a.1] = false;
                let out2 = slide(tar_b, b_now, input, tiles);
                for oi in out2 {
                    out.push(oi);
                }
                return out;
            }
        }
        if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
        {
            // ..
            // ba
            // a_nowとb_nowは想定と違う可能性があるためセットする
            // a_now = (tar_b.0 + 1, tar_b.1 + 1);
            b_now = (tar_b.0 + 1, tar_b.1);
            // 1回回す ただし上のパターンまでで回していたのと逆回り
            // まずinput.n - 2の行に持って行く
            if empty.0 < input.n - 2 {
                for _ in 0..(input.n - 2 - empty.0) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            } else {
                for _ in 0..(empty.0 - input.n - 2) {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
            }
            // bの隣に持って行く
            for _ in 0..(empty.1 - b_now.1) {
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
            }
            // 回す
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            return out;
        }
    }
    // bがあるかチェック
    if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
        || tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
        || tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
        || tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
    {
        // aもあるかチェック
        if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
            || tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
            || tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
            || tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
        {
            // aがある
            if tar_a.0 <= tar_a.1 {
                // 上三角の場合
            } else {
                // 下三角の場合
                unimplemented!();
            }
        } else {
            // aはない
            if tar_a.0 <= tar_a.1 {
                // 上三角の場合
                if tiles[tar_b.0][tar_b.1 + 1] != tree_tiles[tar_b.0][tar_b.1] {
                    // 右上にbがなければ右上にずらす
                    if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1] {
                        // まず空きマスを一番右の列に持って行く
                        for _ in 0..(input.n - 1 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        // 空きマスをbの隣に持って行く
                        for _ in 0..(empty.0 - tar_b.0) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1] {
                        // 空きマスをtar_aまで持って行きたい
                        if empty != tar_a {
                            // 列をinput.n-2に持って行って、行をtar_a.0にする
                            if input.n - 1 == empty.1 {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            } else {
                                for _ in 0..(input.n - 2 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            }
                            for _ in 0..(empty.0 - tar_a.0) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    } else if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1] {
                        if empty == tar_b {
                            // このときだけemptyを左回りに回すと得
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else {
                            // 空きマスを一番右の列に持って行く
                            // 空きマスとbの行が被ってるときだけ迂回
                            if tar_b.0 + 1 == empty.0 && tar_b.1 > empty.1 {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                            for _ in 0..(input.n - 1 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                            // 空きマスをbの右隣まで持って行く
                            if tar_b.0 + 1 > empty.0 {
                                for _ in 0..(tar_b.0 + 1 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            } else {
                                for _ in 0..(empty.0 - tar_b.0 + 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            }
                            // 回す
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                    }
                }
                if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1] {
                    // .b
                    // .. <- aを右下に持ってきて1回回す

                    if a_now.1 < input.n - 1 {
                        // a_nowの右隣まで空きマスを持って行く
                        // aは列から揃える
                        // bを動かしたくないので空きマスは行から揃える
                        if a_now.0 > empty.0 {
                            if a_now.1 < empty.1 {
                                for _ in 0..(a_now.0 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            } else {
                                // a_now.1 > empty.1
                                for _ in 0..(a_now.0 - empty.0 - 1) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                                for _ in 0..(a_now.1 - empty.1 + 1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                        } else {
                            if a_now.1 < empty.1 {
                                for _ in 0..(empty.0 - a_now.0) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            } else {
                                // a_now.1 > empty.1
                                for _ in 0..(a_now.0 - empty.0 - 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                                for _ in 0..(a_now.1 - empty.1 + 1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                        }
                        // 空きマスはa_nowと同じ行であって、a_now.1 <= empty.1となっている
                        for _ in 0..(empty.1 - a_now.1 - 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                        // 空きマスがa_nowの右隣に来たので、aを一番右の列まで持って行く
                        for _ in 0..(input.n - 1 - a_now.1 - 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    // a_now.1 == input.n-1となった
                    if a_now.0 > tar_b.0 + 1 {
                        // a_nowの行を揃える
                        // 空きマスをa_nowの1つ上に持って行きたい
                        // 空きマスは行から揃える
                        if a_now.0 > empty.0 {
                            for _ in 0..(a_now.0 - empty.0 - 1) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                        } else {
                            if a_now.1 != empty.1 {
                                for _ in 0..(empty.0 - a_now.0 + 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            } else {
                                for _ in 0..(empty.0 - a_now.0 - 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                        }
                        // 空きマスの行 == a_now.0 - 1
                        // 空きマスをinput.n - 1列まで持って行く
                        for _ in 0..(input.n - 1 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        // 空きマスがaの1つ上
                        for _ in 0..(tar_b.0 + 1 - a_now.0 - 1) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                    // a_now = (tar_b.0 + 1, tar_b.1 + 1)になった
                    // .b
                    // .a
                    b_now = (tar_b.0, tar_b.1 + 1);
                    // 1回回す
                    // まずinput.n - 2の列に持って行く
                    if empty.1 < input.n - 2 {
                        for _ in 0..(input.n - 2 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                    } else {
                        for _ in 0..(empty.1 - input.n - 2) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                    }
                    // bの隣に持って行く
                    for _ in 0..(empty.0 - b_now.0) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    // 回す
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    return out;
                }
            } else {
                // 下三角の場合
                if tiles[tar_b.0 + 1][tar_b.1] != tree_tiles[tar_b.0][tar_b.1] {
                    // 左下にbがなければ左下にずらす
                    if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1] {
                        // まず空きマスを一番下の行に持って行く
                        for _ in 0..(input.n - 1 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        // 空きマスをbの下に持って行く
                        for _ in 0..(empty.1 - tar_b.1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1] {
                        // 空きマスをtar_aまで持って行きたい
                        if empty != tar_a {
                            // 行をinput.n-2に持って行って、列をtar_a.1にする
                            if input.n - 1 == empty.0 {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            } else {
                                for _ in 0..(input.n - 2 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            }
                            for _ in 0..(empty.1 - tar_a.1) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    } else if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1] {
                        if empty == tar_b {
                            // このときだけemptyを右回りに回すと得
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else {
                            // 空きマスを一番下の行に持って行く
                            // 空きマスとbの列が被ってるときだけ迂回
                            if tar_b.1 + 1 == empty.1 && tar_b.0 > empty.0 {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                            for _ in 0..(input.n - 1 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                            // 空きマスをbの下まで持って行く
                            if tar_b.1 + 1 > empty.1 {
                                for _ in 0..(tar_b.1 + 1 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            } else {
                                for _ in 0..(empty.0 - tar_b.0 + 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            }
                            // 回す
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                    }
                }
                if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1] {
                    // ..
                    // b. <- aを右下に持ってきて1回回す

                    if a_now.0 < input.n - 1 {
                        // a_nowの下まで空きマスを持って行く
                        // aは行から揃える
                        // bを動かしたくないので空きマスは列から揃える
                        if a_now.1 > empty.1 {
                            if a_now.0 < empty.0 {
                                for _ in 0..(a_now.1 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            } else {
                                // a_now.0 > empty.0
                                for _ in 0..(a_now.1 - empty.1 - 1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                                for _ in 0..(a_now.0 - empty.0 + 1) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                        } else {
                            if a_now.0 < empty.0 {
                                for _ in 0..(empty.1 - a_now.1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            } else {
                                // a_now.1 > empty.1
                                for _ in 0..(a_now.1 - empty.1 - 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                                for _ in 0..(a_now.0 - empty.0 + 1) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                        }
                        // 空きマスはa_nowと同じ列であって、a_now.0 <= empty.0となっている
                        for _ in 0..(empty.0 - a_now.0 - 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                        // 空きマスがa_nowの下に来たので、aを一番下の列行で持って行く
                        for _ in 0..(input.n - 1 - a_now.0 - 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    // a_now.0 == input.n-1となった
                    if a_now.1 > tar_b.1 + 1 {
                        // a_nowの列を揃える
                        // 空きマスをa_nowの1つ左に持って行きたい
                        // 空きマスは列から揃える
                        if a_now.1 > empty.1 {
                            for _ in 0..(a_now.1 - empty.1 - 1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                        } else {
                            if a_now.0 != empty.0 {
                                for _ in 0..(empty.1 - a_now.1 + 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            } else {
                                for _ in 0..(empty.1 - a_now.1 - 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                        }
                        // 空きマスの列 == a_now.1 - 1
                        // 空きマスをinput.n - 1列まで持って行く
                        for _ in 0..(input.n - 1 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        // 空きマスがaの1つ左
                        for _ in 0..(tar_b.1 + 1 - a_now.1 - 1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                    // a_now = (tar_b.0 + 1, tar_b.1 + 1)になった
                    // ..
                    // ba
                    b_now = (tar_b.0 + 1, tar_b.1);
                    // 1回回す
                    // まずinput.n - 2の行に持って行く
                    if empty.0 < input.n - 2 {
                        for _ in 0..(input.n - 2 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                    } else {
                        for _ in 0..(empty.0 - input.n - 2) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                    }
                    // bの隣に持って行く
                    for _ in 0..(empty.1 - b_now.1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    // 回す
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    return out;
                }
            }
        }
    }
    // チェック終わり
    let a_now = get_now(tar_a, fix, input, tiles, tree_tiles);
    let out1 = slide(tar_b, a_now, input, tiles);
    for oi in out1 {
        out.push(oi);
    }
    fix[tar_b.0][tar_b.1] = true;
    let b_now = get_now(tar_b, fix, input, tiles, tree_tiles);
    fix[tar_b.0][tar_b.1] = false;
    let out2 = slide(tar_b, b_now, input, tiles);
    for oi in out2 {
        out.push(oi);
    }
    out
}

fn slide(
    tar: (usize, usize),
    now: (usize, usize),
    input: &Input,
    tiles: &mut [Vec<usize>],
) -> Vec<char> {
    let mut now = now;
    // 完成させたタイルには触れない
    // 完成したタイル：自分よりtar_iが小さいか、tar_iが同じだがtar_jが小さいか
    // tar_iは大きいがtar_jがtar_i-1より小さいか
    let start = tar.0.min(tar.1);
    // スライドさせる空きマスの位置を取得
    let mut empty = || -> (usize, usize) {
        for i in start..input.n {
            for j in start..input.n {
                if (i < tar.0 && j == tar.1) || (i == tar.0 && j < tar.1) {
                    continue;
                }
                if 16 == tiles[i][j] {
                    return (i, j);
                }
            }
        }
        (input.n, input.n)
    }();
    eprintln!("tar:{:?}, now:{:?}, empty:{:?}", tar, now, empty);
    for row in tiles.iter() {
        for t in row.iter() {
            eprint!("{:2} ", t);
        }
        eprintln!();
    }

    // now(i,j) を tar(i,j) にスライドさせる
    // (i < tar.0) || (i == tar.0 && j <= tar.1) || (j < start) の
    // タイルは動かさないように注意
    let mut out: Output = vec![];
    // 1個を動かす
    // now(i,j)のiをまず揃え、その次にjを揃える
    if tar.0 > tar.1 {
        // 先に行を揃える場合 tar.0 > tar.1の場合分けで完成したタイルを動かさない
        if tar.0 < now.0 {
            // 空きマスをnow.0の上に持ってくる
            // まず空きマスの列を揃える
            if now.1 < empty.1 {
                // 空きマスが左に行くと完成タイルにぶつかる可能性がある tar.0 > empty.0のとき
                // 空きマスがnowより右にあるとき
                // 同じ行に空きマスとnowがあるとそのまま空きマスとnowを同じ列にはできない
                // 場合分け
                if now.0 == empty.0 {
                    for _ in 0..(empty.1 - now.1 - 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else if tar.0 <= empty.0 {
                    for _ in 0..(empty.1 - now.1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                } else {
                    // tar.0 > empty.0
                    for _ in 0..(empty.1 - now.1 - 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    for _ in 0..(tar.0 - empty.0) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
            } else if now.1 > empty.1 {
                // 空きマスがnowより左にあるとき
                // 同じ行に空きマスとnowがあるとそのまま空きマスとnowを同じ列にはできない
                // 場合分け
                if now.0 == empty.0 {
                    for _ in 0..(now.1 - empty.1 - 1) {
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                } else {
                    // TODO:now.0 < empty.0だとこの後損する
                    // now.1 - empty.1 - 1までRしてその後UUR
                    for _ in 0..(now.1 - empty.1) {
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                }
            }
            // 空きマスの列=nowの列
            // 空きマスの行をnowの上に持ってくる
            if now.0 < empty.0 {
                for _ in 0..(empty.0 - now.0 - 1) {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
                if empty.1 != input.n - 1 {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            } else if now.0 > empty.0 {
                for _ in 0..(now.0 - empty.0 - 1) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            }
            // 空きマスがnowの1つ上に来た
            // nowとtarのiを揃える
            for _ in 0..(now.0 - tar.0 - 1) {
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                now.0 -= 1;
                if now.1 != input.n - 1 {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            }
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            now.0 -= 1;
        } else if tar.0 > now.0 {
            // nowを下に持って行く
            // まず空きマスの列を揃える
            if now.1 < empty.1 {
                // 空きマスがnowより右にあるとき
                // 同じ行に空きマスとnowがあるとそのまま空きマスとnowを同じ列にはできない
                // 場合分け
                if now.0 == empty.0 {
                    for _ in 0..(empty.1 - now.1 - 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else {
                    for _ in 0..(empty.1 - now.1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                }
            } else if now.1 > empty.1 {
                // 空きマスがnowより左にあるとき
                // 同じ行に空きマスとnowがあるとそのまま空きマスとnowを同じ列にはできない
                // 場合分け
                if now.0 == empty.0 {
                    for _ in 0..(now.1 - empty.1 - 1) {
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                } else {
                    for _ in 0..(now.1 - empty.1) {
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                }
            }
            // 空きマスの列=nowの列
            // 空きマスの行をnowの下に持ってくる
            if now.0 < empty.0 {
                for _ in 0..(empty.0 - now.0 - 1) {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
            } else if now.0 > empty.0 {
                for _ in 0..(now.0 - empty.0 - 1) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
                if empty.1 != input.n - 1 {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            }
            // 空きマスがnowの1つ下に来た
            // nowとtarのiを揃える
            for _ in 0..(tar.0 - now.0 - 1) {
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
                now.0 += 1;
                if now.1 != input.n - 1 {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            }
            out.push('U');
            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
            tiles[empty.0 - 1][empty.1] = 16;
            empty.0 -= 1;
            now.0 += 1;
        }
        // tar とnow のiは揃った
        // jを揃える
        if tar.1 < now.1 {
            // now の左に空きマスを持って行く
            // まず空きマスの列を揃える
            if now.1 < empty.1 {
                // 左に行けるなら行きたい
                // now.0 > empty.0 なら完成タイルに当たるかもしれない
                // now.0 == empty.0 なら必ずnowにぶつかるので迂回する
                if now.0 == empty.0 {
                    for _ in 0..(empty.1 - now.1 - 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                } else if now.0 > empty.0 {
                    if tar.1 == now.1 - 1 {
                        for _ in 0..(empty.1 - now.1 - 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                        // 空きマスは左に進めないので迂回
                        for _ in 0..(now.0 - empty.0 + 1) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    } else {
                        for _ in 0..(empty.1 - now.1 + 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                    }
                } else {
                    for _ in 0..(empty.1 - now.1 + 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                }
            } else if now.1 > empty.1 {
                for _ in 0..(now.1 - empty.1 - 1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            } else {
                if now.0 > empty.0 {
                    if tar.1 == now.1 - 1 {
                        // 空きマスは左に進めないので迂回
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                        // 一番下のマスをここで揃えることはないので+1まで空きマスが行って大丈夫
                        for _ in 0..(now.0 - empty.0 + 1) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    } else {
                        for _ in 0..(empty.1 - now.1 + 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                    }
                } else if now.0 < empty.0 {
                    for _ in 0..(empty.1 - now.1 + 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                    }
                } else {
                    // 空きマスとnowが同じ場所にある
                    unreachable!();
                }
            }
            // 空きマスの列=nowの列-1になった
            // 空きマスとnowの行を揃える
            if now.0 > empty.0 {
                for _ in 0..(now.0 - empty.0) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            } else if now.0 < empty.0 {
                for _ in 0..(empty.0 - now.0) {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
            }
            // 空きマスがnowの左に来た
            // nowとtarのjを揃える
            for _ in 0..(now.1 - tar.1 - 1) {
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                now.1 -= 1;
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
            }
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            now.1 -= 1;
        } else if tar.1 > now.1 {
            // あるのか？
            // 今tar.0 > tar.1なのでnow.1 はtar.1以上しか扱えないはず
            unreachable!();
        }
    } else {
        // tar.0 <= tar.1 の場合、列を先に揃える
        // jを揃える
        if tar.1 < now.1 {
            // now の左に空きマスを持って行く
            // まず空きマスの行を揃える
            if now.0 < empty.0 {
                // 空きマスが上に行くと完成タイルにぶつかる可能性がある tar.1 > empty.1のとき
                // 空きマスがnowより下にあるとき
                // 同じ列に空きマスとnowがあるとそのまま空きマスとnowを同じ行にはできない
                // 場合分け
                if now.1 == empty.1 {
                    for _ in 0..(empty.0 - now.0 - 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else if tar.1 <= empty.1 {
                    for _ in 0..(empty.0 - now.0) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                } else {
                    // tar.1 > empty.1
                    for _ in 0..(empty.0 - now.0 - 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    for _ in 0..(tar.1 - empty.1) {
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                }
            } else if now.0 > empty.0 {
                // 空きマスがnowより上にあるとき
                // 同じ列に空きマスとnowがあるとそのまま空きマスとnowを同じ行にはできない
                // 場合分け
                if now.1 == empty.1 {
                    for _ in 0..(now.0 - empty.0 - 1) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                } else {
                    // TODO:now.1 < empty.1だとこの後損する
                    // now.0 - empty.0 - 1までDしてその後LLD
                    for _ in 0..(now.0 - empty.0) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                }
            }
            // 空きマスの行=nowの行となった
            // 空きマスの列をnowの列-1にする
            if now.1 < empty.1 {
                for _ in 0..(empty.1 - now.1 - 1) {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
                if empty.0 != input.n - 1 {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            } else if now.1 > empty.1 {
                for _ in 0..(now.1 - empty.1 - 1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            }
            // 空きマスがnowの左に来た
            // nowとtarのjを揃える
            for _ in 0..(now.1 - tar.1 - 1) {
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                now.1 -= 1;
                if empty.0 != input.n - 1 {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            }
            out.push('R');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
            tiles[empty.0][empty.1 + 1] = 16;
            empty.1 += 1;
            now.1 -= 1;
        } else if tar.1 > now.1 {
            // now の右に空きマスを持って行く
            // まず空きマスの行を揃える
            if now.0 < empty.0 {
                // 空きマスがnowより下にあるとき
                // 同じ列に空きマスとnowがあるとそのまま空きマスとnowを同じ行にはできない
                // 場合分け
                if now.1 == empty.1 {
                    for _ in 0..(empty.0 - now.0 - 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else {
                    for _ in 0..(empty.0 - now.0) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                }
            } else if now.0 > empty.0 {
                // 空きマスがnowより上にあるとき
                // 同じ列に空きマスとnowがあるとそのまま空きマスとnowを同じ行にはできない
                // 場合分け
                if now.1 == empty.1 {
                    for _ in 0..(now.0 - empty.0 - 1) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                } else {
                    for _ in 0..(now.0 - empty.0) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                    }
                }
            }
            // 空きマスの行 == nowの行
            // 空きマスの列をnowの列+1にする
            if now.1 < empty.1 {
                for _ in 0..(empty.1 - now.1 - 1) {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
            } else if now.1 > empty.1 {
                for _ in 0..(now.1 - empty.1 - 1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
                if empty.0 != input.n - 1 {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            }
            // 空きマスがnowの右に来た
            // nowとtarのjを揃える
            for _ in 0..(tar.1 - now.1 - 1) {
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
                now.1 += 1;
                if empty.0 != input.n - 1 {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else {
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            }
            out.push('L');
            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
            tiles[empty.0][empty.1 - 1] = 16;
            empty.1 -= 1;
            now.1 += 1;
        }
        // tarとnowのjが揃った
        // iを揃える
        if tar.0 < now.0 {
            // now の上に空きマスを持って行く
            // まず空きマスの行を揃える
            if now.0 < empty.0 {
                // 上に行けるなら行きたい
                // now.1 > empty.1 なら完成タイルに当たるかもしれない
                // now.1 == empty.1 なら必ずnowにぶつかるので迂回する
                if now.1 == empty.1 {
                    for _ in 0..(empty.0 - now.0 - 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                    out.push('U');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
                } else if now.1 > empty.1 {
                    if tar.0 == now.0 - 1 {
                        for _ in 0..(empty.0 - now.0 - 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                        // 空きマスは上に進めないので迂回
                        for _ in 0..(now.1 - empty.1 + 1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    } else {
                        for _ in 0..(empty.0 - now.0 + 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                    }
                } else {
                    for _ in 0..(empty.1 - now.1 + 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                }
            } else if now.0 > empty.0 {
                for _ in 0..(now.0 - empty.0 - 1) {
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                }
            } else {
                if now.1 > empty.1 {
                    if tar.0 == now.0 - 1 {
                        // 空きマスは上に進めないので迂回
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                        // 一番右のマスをここで揃えることはないので+1まで空きマスが行って大丈夫
                        for _ in 0..(now.1 - empty.1 + 1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    } else {
                        for _ in 0..(empty.0 - now.0 + 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                    }
                } else if now.1 < empty.1 {
                    for _ in 0..(empty.0 - now.0 + 1) {
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                } else {
                    // 空きマスとnowが同じ場所にある
                    unreachable!();
                }
            }
            // 空きマスの行=nowの行-1になった
            // 空きマスとnowの列を揃える
            if now.1 > empty.1 {
                for _ in 0..(now.1 - empty.1) {
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                }
            } else if now.1 < empty.1 {
                for _ in 0..(empty.1 - now.1) {
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                    tiles[empty.0][empty.1 - 1] = 16;
                    empty.1 -= 1;
                }
            }
            // 空きマスがnowの上に来た
            // nowとtarのiを揃える
            for _ in 0..(now.0 - tar.0 - 1) {
                out.push('D');
                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                tiles[empty.0 + 1][empty.1] = 16;
                empty.0 += 1;
                now.0 -= 1;
                out.push('R');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                tiles[empty.0][empty.1 + 1] = 16;
                empty.1 += 1;
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
                out.push('U');
                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                tiles[empty.0 - 1][empty.1] = 16;
                empty.0 -= 1;
                out.push('L');
                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                tiles[empty.0][empty.1 - 1] = 16;
                empty.1 -= 1;
            }
            out.push('D');
            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
            tiles[empty.0 + 1][empty.1] = 16;
            empty.0 += 1;
            now.0 -= 1;
        } else if tar.0 > now.0 {
            // あるのか？
            // 今tar.0 <= tar.1なのでnow.0 はtar.0以上しか扱えないはず
            unreachable!();
        }
    }
    out
}

fn is_empty_space(input: &Input, now_tiles: &[Vec<usize>]) -> bool {
    // 455900
    // c9c34b
    // a6380a <- ここの0みたいなのを空きスペースと言った
    // 698edb
    // 067300
    // 000000
    // こういうのがあったらtrue

    // うわーついでにこういうのも空きスペースと呼びたい
    // 455900 <- この右上に絶対にいけない
    // c9c349
    // a6384b
    // 698edb
    // 067300
    // 000000

    // もっとやばい
    // 800880
    // 655ba0
    // 00c3a0
    // 00acb0
    // 006ba0
    // 000200

    // 0からBFSしてどこか開いているところにつながればOK
    // つながらなかったらデッドスペースができている
    // 0からBFSすれば4x4とかも全部見れるのでは？？？
    let mut que = VecDeque::new();
    let mut visited = vec![vec![false; input.n]; input.n];
    for si in 0..input.n {
        for sj in 0..input.n {
            if now_tiles[si][sj] != 0 {
                continue;
            }
            if visited[si][sj] {
                continue;
            }
            visited[si][sj] = true;
            que.push_back((si, sj));
            let mut is_open = false; // 0だけで到達できる連結成分のどこかが開いているかチェック
            while !que.is_empty() {
                let v = que.pop_front().unwrap();
                for (d, (di, dj)) in DIJ.iter().enumerate() {
                    let ni = v.0 + *di;
                    let nj = v.1 + *dj;
                    if input.n <= ni || input.n <= nj {
                        continue;
                    }
                    if visited[ni][nj] {
                        continue;
                    }
                    if now_tiles[ni][nj] == 0 {
                        visited[ni][nj] = true;
                        que.push_back((ni, nj));
                    } else if (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 1 {
                        // 開いているかチェック
                        is_open = true;
                    }
                }
            }
            if !is_open {
                return true;
            }
        }
    }

    false
}

fn dfs(
    pos: (usize, usize),
    input: &Input,
    tile_count: &mut [i32],
    now_tiles: &mut [Vec<usize>],
    next_poses: &mut Vec<(usize, usize)>,
    tile_is: &[Vec<Vec<usize>>],
    count: &mut usize,
) -> bool {
    // 今のposに置くタイルを決める
    for &tile_i in tile_is[pos.0][pos.1].iter() {
        if tile_count[tile_i] == 0 {
            // このタイルはない
            continue;
        }
        let mut open_count = 0;
        let mut can_empty_space = false;
        let mut can_empty_space2 = false;
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
                        open_count += 1;
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
                // 今置くタイルが開いていない方向の別のタイルが、置いてあって（!=0）、開いていないなら
                // 空きスペースを作った可能性がある
                // 455900
                // c9c34b
                // a6380a <- ここの0みたいなのを空きスペースと言った
                // 698edb
                // 067300
                // 000000
                // 空きスペースチェックを走らせて、空きスペースが本当にあったらfalseを返したい
                if i2 < input.n
                    && j2 < input.n
                    && now_tiles[i2][j2] != 0
                    && (now_tiles[i2][j2] >> (d ^ 2)) & 1 == 0
                {
                    can_empty_space = true;
                }
                // こういう空きスペースも検知したい
                // 800880
                // 655ba0
                // 00c3a0
                // 00acb0
                // 006ba0
                // 000200
                // 壁に接するところに置いたときで、どこにも開いてなければ走らせるとよさげ？
                if i2 >= input.n || j2 >= input.n {
                    can_empty_space2 = true;
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
                if (can_empty_space || open_count == 0 && can_empty_space2)
                    && is_empty_space(input, now_tiles)
                {
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                    continue;
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
                        next_poses,
                        tile_is,
                        count,
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
                    if (can_empty_space || open_count == 0 && can_empty_space2)
                        && is_empty_space(input, now_tiles)
                    {
                        now_tiles[pos.0][pos.1] = 0;
                        tile_count[tile_i] += 1;
                        continue;
                    }
                    if dfs(
                        (i2, j2),
                        input,
                        tile_count,
                        now_tiles,
                        next_poses,
                        tile_is,
                        count,
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
    *count += 1;
    // if *count <= 1000 && *count % 100 == 0 {
    //     eprintln!("count: {}", count);
    //     eprintln!("{:?}", tile_count);
    //     for row in now_tiles.iter() {
    //         for t in row.iter() {
    //             if *t == 16 {
    //                 eprint!("{:x}", 0);
    //             } else {
    //                 eprint!("{:x}", t);
    //             }
    //         }
    //         eprintln!();
    //     }
    // }
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
