#![allow(
    clippy::needless_range_loop,
    clippy::comparison_chain,
    clippy::collapsible_else_if,
    clippy::same_item_push
)]
use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::vec;

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
    if dfs(
        (0, 0),
        &input,
        &mut tile_count,
        &mut now_tiles,
        &mut next_poses,
    ) {
        for row in now_tiles.iter() {
            for t in row.iter() {
                print!("{:2} ", t);
            }
            println!();
        }
    }

    // 見つけた木となるような操作列の構築
    construct(&input, &mut now_tiles);
}

fn get_now(
    start: usize,
    tar: (usize, usize),
    fix: &[Vec<bool>],
    input: &Input,
    tiles: &[Vec<usize>],
    tree_tiles: &[Vec<usize>],
) -> (usize, usize) {
    for i in start..input.n {
        for j in start..input.n {
            if fix[i][j] {
                continue;
            }
            if tree_tiles[tar.0][tar.1] == tiles[i][j] {
                return (i, j);
            }
        }
    }
    println!("{} {}", tar.0, tar.1);
    unreachable!("not found: now:(");
}

fn construct(input: &Input, tree_tiles: &mut [Vec<usize>]) {
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
                let now = get_now(i, (i, j), &fix, input, &tiles, tree_tiles);
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
                let now = get_now(i, (i2, i), &fix, input, &tiles, tree_tiles);
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
    println!("{}", out.iter().join(""));
    // 3*3を完成させる
    for row in tiles.iter() {
        for t in row.iter() {
            print!("{:2} ", t);
        }
        println!();
    }
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
    let a_now = get_now(start, tar_a, fix, input, tiles, tree_tiles);
    let out1 = slide(tar_b, a_now, input, tiles);
    for oi in out1 {
        out.push(oi);
    }
    fix[tar_b.0][tar_b.1] = true;
    let b_now = get_now(start, tar_b, fix, input, tiles, tree_tiles);
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
                    for _ in 0..(empty.0 - now.0 - 1) {
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                    }
                    for _ in 0..(tar.1 - empty.1) {
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                    }
                    out.push('L');
                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                    tiles[empty.0 - 1][empty.1] = 16;
                    empty.0 -= 1;
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
                    if tar.1 == now.1 - 1 {
                        // 空きマスは左に進めないので迂回
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

fn dfs(
    pos: (usize, usize),
    input: &Input,
    tile_count: &mut [i32],
    now_tiles: &mut [Vec<usize>],
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
                    if dfs((i2, j2), input, tile_count, now_tiles, next_poses) {
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
