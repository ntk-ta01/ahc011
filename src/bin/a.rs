#![allow(
    clippy::needless_range_loop,
    clippy::comparison_chain,
    clippy::collapsible_else_if,
    clippy::same_item_push,
    clippy::too_many_arguments
)]
use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::{input, marker::Chars};
// use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::exit,
};

pub type Output = Vec<char>;

pub const DIJ: [(usize, usize); 4] = [(0, !0), (!0, 0), (0, 1), (1, 0)];
pub const DIR: [char; 4] = ['L', 'U', 'R', 'D'];
const TIMELIMIT: f64 = 2.7;

pub struct Input {
    pub n: usize,
    pub t: usize,
    pub tiles: Vec<Vec<usize>>,
}
fn main() {
    let timer = Timer::new();
    // let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(93216000);
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
    let arr = [7, 11, 13, 14, 15];

    let mut now_tiles = vec![vec![0; input.n]; input.n];
    now_tiles[input.n - 1][input.n - 1] = 16;
    let mut next_poses = vec![];

    for j in (0..input.n).rev() {
        for i in 0..input.n {
            // 次数3以上を固定する
            let fix_tile_i = input.tiles[i][j];
            if arr.iter().all(|a| *a != fix_tile_i) {
                continue;
            }
            // eprintln!("(i, j), ({} {})", i, j);
            if i == input.n - 1 && j == input.n - 1 {
                continue;
            }
            // TODO:不可能な組合せには置きたくない
            // まず開いているか確認
            match fix_tile_i {
                7 => {
                    if i == 0 || j == 0 || j == input.n - 1 {
                        continue;
                    }
                    if tile_count[7] == 0 {
                        continue;
                    }
                    if now_tiles[i][j] != 0 {
                        continue;
                    }
                    if now_tiles[i][j - 1] != 0 && (now_tiles[i][j - 1] >> 2) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i - 1][j] != 0 && (now_tiles[i - 1][j] >> 3) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i][j + 1] != 0 && now_tiles[i][j + 1] & 1 != 1 {
                        continue;
                    }
                    if i != input.n - 1
                        && now_tiles[i + 1][j] != 0
                        && (now_tiles[i][j - 1] >> 1) & 1 != 0
                    {
                        continue;
                    }
                    now_tiles[i][j] = 7;
                    tile_count[7] -= 1;
                    next_poses.push((i, j - 1));
                    next_poses.push((i - 1, j));
                    next_poses.push((i, j + 1));
                }
                11 => {
                    if i == 0 || j == 0 || i == input.n - 1 {
                        continue;
                    }
                    if tile_count[11] == 0 {
                        continue;
                    }
                    if now_tiles[i][j] != 0 {
                        continue;
                    }
                    if now_tiles[i][j - 1] != 0 && (now_tiles[i][j - 1] >> 2) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i - 1][j] != 0 && (now_tiles[i - 1][j] >> 3) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i + 1][j] != 0 && (now_tiles[i][j - 1] >> 1) & 1 != 1 {
                        continue;
                    }
                    if j != input.n - 1 && now_tiles[i][j + 1] != 0 && now_tiles[i][j + 1] & 1 != 0
                    {
                        continue;
                    }
                    now_tiles[i][j] = 11;
                    tile_count[11] -= 1;
                    next_poses.push((i, j - 1));
                    next_poses.push((i - 1, j));
                    next_poses.push((i + 1, j));
                }
                13 => {
                    if i == input.n - 1 || j == 0 || j == input.n - 1 {
                        continue;
                    }
                    if tile_count[13] == 0 {
                        continue;
                    }
                    if now_tiles[i][j] != 0 {
                        continue;
                    }
                    if now_tiles[i][j - 1] != 0 && (now_tiles[i][j - 1] >> 2) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i][j + 1] != 0 && now_tiles[i][j + 1] & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i + 1][j] != 0 && (now_tiles[i][j - 1] >> 1) & 1 != 1 {
                        continue;
                    }
                    if i != 0 && now_tiles[i - 1][j] != 0 && (now_tiles[i - 1][j] >> 3) & 1 != 0 {
                        continue;
                    }
                    now_tiles[i][j] = 13;
                    tile_count[13] -= 1;
                    next_poses.push((i, j - 1));
                    next_poses.push((i, j + 1));
                    next_poses.push((i + 1, j));
                }
                14 => {
                    if i == 0 || i == input.n - 1 || j == 0 || j == input.n - 1 {
                        continue;
                    }
                    if tile_count[14] == 0 {
                        continue;
                    }
                    if now_tiles[i][j] != 0 {
                        continue;
                    }
                    if now_tiles[i - 1][j] != 0 && (now_tiles[i - 1][j] >> 3) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i][j + 1] != 0 && now_tiles[i][j + 1] & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i + 1][j] != 0 && (now_tiles[i][j - 1] >> 1) & 1 != 1 {
                        continue;
                    }
                    if j != 0 && now_tiles[i][j - 1] != 0 && (now_tiles[i][j - 1] >> 2) & 1 != 0 {
                        continue;
                    }
                    now_tiles[i][j] = 14;
                    tile_count[14] -= 1;
                    next_poses.push((i - 1, j));
                    next_poses.push((i, j + 1));
                    next_poses.push((i + 1, j));
                }
                15 => {
                    if i == 0 || i == input.n - 1 || j == 0 || j == input.n - 1 {
                        continue;
                    }
                    if tile_count[15] == 0 {
                        continue;
                    }
                    if now_tiles[i][j] != 0 {
                        continue;
                    }
                    if now_tiles[i][j - 1] != 0 && (now_tiles[i][j - 1] >> 2) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i - 1][j] != 0 && (now_tiles[i - 1][j] >> 3) & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i][j + 1] != 0 && now_tiles[i][j + 1] & 1 != 1 {
                        continue;
                    }
                    if now_tiles[i + 1][j] != 0 && (now_tiles[i][j - 1] >> 1) & 1 != 1 {
                        continue;
                    }
                    now_tiles[i][j] = 15;
                    tile_count[15] -= 1;
                    next_poses.push((i, j - 1));
                    next_poses.push((i - 1, j));
                    next_poses.push((i, j + 1));
                    next_poses.push((i + 1, j));
                }
                _ => unreachable!(),
            }
        }
    }
    let mut last_tiles = vec![vec![0; input.n]; input.n];
    let tile_is = get_tile_is(&input);
    for _turn in 1.. {
        if TIMELIMIT < timer.get_time() {
            // eprintln!("turn: {}", turn);
            // eprintln!("{} {}", input.n, input.t);
            // for row in now_tiles.iter() {
            //     for t in row.iter() {
            //         if *t == 16 {
            //             eprint!("{:x}", 0);
            //         } else {
            //             eprint!("{:x}", t);
            //         }
            //     }
            //     eprintln!();
            // }
            break;
        }
        let mut count = 0;
        let mut fails = vec![vec![0; input.n]; input.n];
        if dfs(
            (0, 0),
            &input,
            &mut tile_count,
            &mut now_tiles,
            &mut next_poses,
            &tile_is,
            &mut count,
            &mut fails,
            &mut last_tiles,
            &timer,
        ) {
            // eprintln!("count: {}", count);
            // for row in now_tiles.iter() {
            //     for t in row.iter() {
            //         eprint!("{:2} ", t);
            //     }
            //     eprintln!();
            // }
            return;
        }
        // 作れなかった場合のnow_tilesから教訓を得たい
        // for row in fails.iter() {
        //     for f in row.iter() {
        //         eprint!("{:7} ", f);
        //     }
        //     eprintln!();
        // }
        // now_tilesを調整
        // 1に挟まれている奴をfix
        // 縦のチェック
        for i in 1..input.n - 1 {
            for j in 0..input.n {
                if 0 < fails[i - 1][j]
                    && fails[i - 1][j] <= 3
                    && 0 < fails[i + 1][j]
                    && fails[i + 1][j] <= 3
                    && last_tiles[i][j] != 0
                    && 0 < tile_count[last_tiles[i][j]]
                {
                    now_tiles[i][j] = last_tiles[i][j];
                    tile_count[now_tiles[i][j]] -= 1;
                }
            }
        }
        // 横のチェック
        for i in 0..input.n {
            for j in 1..input.n - 1 {
                if 0 < fails[i][j - 1]
                    && fails[i][j - 1] <= 3
                    && 0 < fails[i][j + 1]
                    && fails[i][j + 1] <= 3
                    && last_tiles[i][j] != 0
                    && 0 < tile_count[last_tiles[i][j]]
                {
                    now_tiles[i][j] = last_tiles[i][j];
                    tile_count[now_tiles[i][j]] -= 1;
                }
            }
        }
        // eprintln!("{} {}", input.n, input.t);
        // for row in now_tiles.iter() {
        //     for t in row.iter() {
        //         if *t == 16 {
        //             eprint!("{:x}", 0);
        //         } else {
        //             eprint!("{:x}", t);
        //         }
        //     }
        //     eprintln!();
        // }
        let mut moved = vec![vec![false; input.n]; input.n];
        // 悪い奴を破壊する 破壊？前後左右どこかに移動 タイルがあったらswap
        for i in 0..input.n {
            for j in 0..input.n {
                if moved[i][j] {
                    continue;
                }
                if arr.iter().any(|ai| *ai == now_tiles[i][j]) {
                    // 1に挟まれてても
                    // 4近傍でめちゃくちゃに失敗しているとき、移動させる
                    // 移動先はつながるように選ぶ、どこにも移動できなかったら削除する
                    let mut max_fail = 0;
                    let mut max_i = 0;
                    for (d, (di, dj)) in DIJ.iter().enumerate() {
                        let ni = i + *di;
                        let nj = j + *dj;
                        if input.n <= ni || input.n <= nj {
                            continue;
                        }
                        if max_fail < fails[ni][nj] {
                            max_fail = fails[ni][nj];
                            max_i = d;
                        }
                    }
                    if 100 <= max_fail {
                        // 移動 or 削除の対象
                        // まずmax_iに移動できるか試す
                        // タイル番号が同じ奴とswapしても仕方ない
                        // 移動先がfails=1でもつながってたらおっけー
                        // fails = 0とはつながっている必要ある？いったんあるにしておくか
                        let mut is_place = true;
                        let ni = i + DIJ[max_i].0;
                        let nj = j + DIJ[max_i].1;
                        if now_tiles[ni][nj] == 0 {
                            for (d, (di, dj)) in DIJ.iter().enumerate() {
                                // now_tiles[i][j]を置けるかチェック
                                // 開いている方向が壁でなく、空きマスか他のマスの開いている方向か見る
                                let ni2 = ni + *di;
                                let nj2 = nj + *dj;
                                if (now_tiles[i][j] >> d) & 1 == 1 {
                                    if ni2 >= input.n
                                        || nj2 >= input.n
                                        || now_tiles[ni2][nj2] != 0
                                            && d ^ 2 != max_i
                                            && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 0
                                    {
                                        // 別のタイルを置いているマスであって、そのタイルが開いていない方向に
                                        // now_tiles[i][j]が開いていると置けない
                                        // d^2 != max_i は今タイルの置いていないマスと交換しようとしているので
                                        // now_tiles[ni][nj] == 0なら
                                        // 元来た方向を見るとき（d^2 == max_i）は必ず条件を満たす
                                        is_place = false;
                                    }
                                } else {
                                    if ni2 < input.n
                                        && nj2 < input.n
                                        && d ^ 2 != max_i
                                        && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 1
                                    {
                                        // now_tilesが開いていない方向が、壁じゃないかつ
                                        // 別のタイルの開いている方向なら置けない
                                        is_place = false;
                                    }
                                }
                            }
                            if is_place {
                                // 移動させる
                                now_tiles[ni][nj] = now_tiles[i][j];
                                now_tiles[i][j] = 0;
                                moved[ni][nj] = true;
                                moved[i][j] = true;
                            }
                        } else if now_tiles[ni][nj] != 16 {
                            // 空きマスでない 16も動かしたくない
                            for (d, (di, dj)) in DIJ.iter().enumerate() {
                                // now_tiles[i][j]を置けるかチェック
                                // 開いている方向が壁でなく、空きマスか他のマスの開いている方向か見る
                                let ni2 = ni + *di;
                                let nj2 = nj + *dj;
                                if (now_tiles[i][j] >> d) & 1 == 1 {
                                    if ni2 >= input.n
                                        || nj2 >= input.n
                                        || now_tiles[ni2][nj2] != 0
                                            && d ^ 2 != max_i
                                            && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 0
                                    {
                                        // 別のタイルを置いているマスであって、そのタイルが開いていない方向に
                                        // now_tiles[i][j]が開いていると置けない
                                        is_place = false;
                                    }
                                    if ni2 >= input.n
                                        || nj2 >= input.n
                                        || now_tiles[ni][nj] != 0
                                            && d ^ 2 == max_i
                                            && (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 0
                                    {
                                        // swapするタイルとつながるか
                                        is_place = false;
                                    }
                                } else {
                                    if ni2 < input.n
                                        && nj2 < input.n
                                        && d ^ 2 != max_i
                                        && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 1
                                    {
                                        // now_tilesが開いていない方向が、壁じゃないかつ
                                        // 別のタイルの開いている方向なら置けない
                                        is_place = false;
                                    }
                                    if ni2 < input.n
                                        && nj2 < input.n
                                        && d ^ 2 == max_i
                                        && (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 1
                                    {
                                        // now_tilesが開いていない方向が、壁じゃないかつ
                                        // swapするタイルの開いている方向なら置けない
                                        is_place = false;
                                    }
                                }
                            }
                            if is_place {
                                // swapさせる
                                let tmp = now_tiles[ni][nj];
                                now_tiles[ni][nj] = now_tiles[i][j];
                                now_tiles[i][j] = tmp;
                                moved[ni][nj] = true;
                                moved[i][j] = true;
                            }
                        } else {
                            is_place = false;
                        }
                        if !is_place {
                            // max_i以外の方向に動かせるか調べる
                            for other_i in 0..4 {
                                if other_i == max_i {
                                    continue;
                                }
                                let ni = i + DIJ[other_i].0;
                                let nj = j + DIJ[other_i].1;
                                if ni >= input.n || nj >= input.n {
                                    continue;
                                }
                                if now_tiles[ni][nj] == 16 {
                                    continue;
                                }
                                is_place = true;
                                // 次数3以上で上手くいってそうなタイルは動かしたくない
                                // そういうタイルの周りはそんなに失敗していないと思いたい
                                // いやそんなことはないか……
                                // 交換先のタイルが次数3以上で
                                // 3つ以上fails <= 3があったら交換しない
                                let mut uoo_count = 0;
                                for (d, (di, dj)) in DIJ.iter().enumerate() {
                                    // now_tiles[i][j]を置けるかチェック
                                    // 開いている方向が壁でなく、空きマスか他のマスの開いている方向か見る
                                    let ni2 = ni + *di;
                                    let nj2 = nj + *dj;
                                    if ni2 < input.n
                                        && nj2 < input.n
                                        && arr.iter().any(|ai| now_tiles[ni][nj] == *ai)
                                        && fails[ni2][nj2] <= 3
                                    {
                                        uoo_count += 1;
                                    }
                                    if (now_tiles[i][j] >> d) & 1 == 1 {
                                        if ni2 >= input.n
                                            || nj2 >= input.n
                                            || now_tiles[ni2][nj2] != 0
                                                && d ^ 2 != other_i
                                                && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 0
                                        {
                                            // 別のタイルを置いているマスであって、そのタイルが開いていない方向に
                                            // now_tiles[i][j]が開いていると置けない
                                            // d^2 != max_i は今タイルの置いていないマスと交換しようとしているので
                                            // now_tiles[ni][nj] == 0なら
                                            // 元来た方向を見るとき（d^2 == max_i）は必ず条件を満たす
                                            is_place = false;
                                        }
                                        if now_tiles[ni][nj] != 16
                                            && now_tiles[ni][nj] != 0
                                            && (ni2 >= input.n
                                                || nj2 >= input.n
                                                || now_tiles[ni][nj] != 0
                                                    && d ^ 2 == other_i
                                                    && (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 0)
                                        {
                                            // swapするタイルとつながるか
                                            is_place = false;
                                        }
                                    } else {
                                        if ni2 < input.n
                                            && nj2 < input.n
                                            && d ^ 2 != max_i
                                            && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 1
                                        {
                                            // now_tilesが開いていない方向が、壁じゃないかつ
                                            // 別のタイルの開いている方向なら置けない
                                            is_place = false;
                                        }
                                        if now_tiles[ni][nj] != 16
                                            && now_tiles[ni][nj] != 0
                                            && (ni2 < input.n
                                                && nj2 < input.n
                                                && d ^ 2 == other_i
                                                && (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 1)
                                        {
                                            // now_tilesが開いていない方向が、壁じゃないかつ
                                            // swapするタイルの開いている方向なら置けない
                                            is_place = false;
                                        }
                                    }
                                }
                                if uoo_count >= 3 {
                                    is_place = false;
                                }
                                if is_place {
                                    // swapさせる
                                    let tmp = now_tiles[ni][nj];
                                    now_tiles[ni][nj] = now_tiles[i][j];
                                    now_tiles[i][j] = tmp;
                                    moved[ni][nj] = true;
                                    moved[i][j] = true;
                                    break;
                                }
                            }
                        }
                        if !is_place {
                            // 置けなかったので排除する
                            tile_count[now_tiles[i][j]] += 1;
                            now_tiles[i][j] = 0;
                        }
                    }
                }
            }
        }
        for i in 0..input.n {
            for j in 0..input.n {
                if now_tiles[i][j] == 0 {
                    let mut need_tile = 0;
                    let mut adds = vec![0];
                    for (d, (di, dj)) in DIJ.iter().enumerate() {
                        let ni = i + *di;
                        let nj = j + *dj;
                        if ni >= input.n || nj >= input.n {
                            continue;
                        }
                        if now_tiles[ni][nj] == 0 {
                            adds.push(1 << d);
                            continue;
                        }
                        if (now_tiles[ni][nj] >> (d ^ 2)) & 1 == 1 {
                            need_tile |= 1 << d;
                        }
                    }
                    if adds.iter().all(|b| tile_count[need_tile | *b] == 0) {
                        // now_tiles[i][j]に置くようなタイルがない
                        // ので周りを削除する
                        for (di, dj) in DIJ.iter() {
                            let ni = i + *di;
                            let nj = j + *dj;
                            if ni >= input.n || nj >= input.n {
                                continue;
                            }
                            if now_tiles[ni][nj] == 0 {
                                continue;
                            }
                            if now_tiles[ni][nj] == 16 {
                                continue;
                            }
                            tile_count[now_tiles[ni][nj]] += 1;
                            now_tiles[ni][nj] = 0;
                        }
                    } else if adds.len() == 1 {
                        // addsのlenが1なら一通りだから置いちゃう方がよくないか？
                        now_tiles[i][j] = need_tile;
                        tile_count[need_tile] -= 1;
                    }
                }
            }
        }
        // eprintln!("now");
        // eprintln!("{} {}", input.n, input.t);
        // for row in now_tiles.iter() {
        //     for t in row.iter() {
        //         if *t == 16 {
        //             eprint!("{:x}", 0);
        //         } else {
        //             eprint!("{:x}", t);
        //         }
        //     }
        //     eprintln!();
        // }
        // eprintln!("last");
        // eprintln!("{} {}", input.n, input.t);
        // for row in last_tiles.iter() {
        //     for t in row.iter() {
        //         if *t == 16 {
        //             eprint!("{:x}", 0);
        //         } else {
        //             eprint!("{:x}", t);
        //         }
        //     }
        //     eprintln!();
        // }
        // now_tilesにあるやつを移動させる
        let ord = vec![0, 1, 2, 3];
        let mut moved = vec![vec![false; input.n]; input.n];
        for i in 0..input.n {
            for j in 0..input.n {
                if moved[i][j] {
                    continue;
                }
                if now_tiles[i][j] != 0
                    && now_tiles[i][j] != 16
                    && arr.iter().any(|ai| *ai == now_tiles[i][j])
                {
                    // 次数3以上なら必ず移動
                    // 移動できなければ削除
                    let mut is_place;
                    for &other_i in ord.iter() {
                        let ni = i + DIJ[other_i].0;
                        let nj = j + DIJ[other_i].1;
                        if ni >= input.n || nj >= input.n {
                            continue;
                        }
                        if now_tiles[ni][nj] == 16 {
                            continue;
                        }
                        is_place = true;
                        for (d, (di, dj)) in DIJ.iter().enumerate() {
                            // now_tiles[i][j]を置けるかチェック
                            // 開いている方向が壁でなく、空きマスか他のマスの開いている方向か見る
                            let ni2 = ni + *di;
                            let nj2 = nj + *dj;
                            if (now_tiles[i][j] >> d) & 1 == 1 {
                                if ni2 >= input.n
                                    || nj2 >= input.n
                                    || now_tiles[ni2][nj2] != 0
                                        && (d ^ 2) != other_i
                                        && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 0
                                {
                                    is_place = false;
                                }
                            } else {
                                if ni2 < input.n
                                    && nj2 < input.n
                                    && d ^ 2 != other_i
                                    && (now_tiles[ni2][nj2] >> (d ^ 2)) & 1 == 1
                                {
                                    // now_tilesが開いていない方向が、壁じゃないかつ
                                    // 別のタイルの開いている方向なら置けない
                                    is_place = false;
                                }
                            }
                        }
                        if is_place {
                            // swapさせる
                            let tmp = now_tiles[ni][nj];
                            now_tiles[ni][nj] = now_tiles[i][j];
                            now_tiles[i][j] = tmp;
                            moved[ni][nj] = true;
                            moved[i][j] = true;
                            break;
                        }
                    }
                }
            }
        }
        // next_posesを作り直す
        next_poses.clear();
        for i in 0..input.n {
            for j in 0..input.n {
                if now_tiles[i][j] != 0 && now_tiles[i][j] != 16 {
                    for (d, (di, dj)) in DIJ.iter().enumerate() {
                        let ni = i + *di;
                        let nj = j + *dj;
                        if ni >= input.n || nj >= input.n {
                            continue;
                        }
                        if now_tiles[ni][nj] == 0 || now_tiles[ni][nj] == 16 {
                            continue;
                        }
                        if (now_tiles[i][j] >> d) & 1 == 0 {
                            continue;
                        }
                        if fails[ni][nj] == 1_000_000_007 {
                            // next_posesに挿入済み
                            // failsもう使わないから使いまわした
                            continue;
                        }
                        next_poses.push((ni, nj));
                        fails[ni][nj] = 1_000_000_007;
                    }
                }
            }
        }
    }
    // 見つけた木となるような操作列の構築 もdfsの中でやる↑
    // let out = construct(&input, &mut now_tiles);
    // println!("{}", out.iter().take(input.t).join(""));
    // 木が見つからなかったとき、適当に敷き詰めて解く
    // tile_countをlast_tile要に作り変える
    let last_tile_count = {
        let mut count = vec![0; 17];
        for row in last_tiles.iter() {
            for t in row.iter() {
                count[*t] += 1;
            }
        }
        count
    };
    let mut in_tile_count = {
        let mut count = vec![0; 17];
        for row in input.tiles.iter() {
            for t in row.iter() {
                count[*t] += 1;
            }
        }
        count
    };
    for i in 1..16 {
        in_tile_count[i] -= last_tile_count[i];
    }
    let mut tile_i = 1;
    for i in 0..input.n {
        for j in 0..input.n {
            if last_tiles[i][j] == 0 {
                while in_tile_count[tile_i] == 0 {
                    tile_i += 1;
                    if tile_i == 16 {
                        tile_i = 1;
                    }
                }
                last_tiles[i][j] = tile_i;
                in_tile_count[tile_i] -= 1;
                tile_i += 1;
                if tile_i == 16 {
                    tile_i = 1;
                }
            }
        }
    }
    // eprintln!("last: ");
    // for row in last_tiles.iter() {
    //     for t in row.iter() {
    //         eprint!("{:2} ", t);
    //     }
    //     eprintln!();
    // }
    if let Some(out) = construct(&input, &mut last_tiles) {
        println!("{}", out.iter().join(""));
    } else {
        println!();
    }
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

fn construct(input: &Input, tree_tiles: &mut [Vec<usize>]) -> Option<Output> {
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
    if let Some(out_i) = slide3x3(input, &mut tiles, tree_tiles) {
        for oi in out_i {
            out.push(oi);
        }
    } else {
        return None;
    }
    // 3x3でtilesを動かしてないのでここで出力しても一見そろってない
    // が、goalしていたらoutは揃っている
    Some(out)
}

fn slide3x3(
    input: &Input,
    tiles: &mut [Vec<usize>],
    tree_tiles: &mut [Vec<usize>],
) -> Option<Output> {
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

    if !parity_check(input, &tiles3x3, tree_tiles) {
        return None;
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
        return Some(out);
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
                // eprintln!("goal");
                break 'lp;
            }
        }
    }

    // 経路復元
    let mut empty = get_empty(&goal);
    let mut v = g;
    let mut v_tiles = goal;
    while v != s {
        let pi = map.get(&v)?;
        let i = prev[*pi];

        out.push(DIR[i]);
        let ni = empty.0 + DIJ[i ^ 2].0;
        let nj = empty.1 + DIJ[i ^ 2].1;
        v_tiles.swap(empty.0 * 3 + empty.1, ni * 3 + nj);
        v = tiles2num(&v_tiles);
        empty = (ni, nj);
    }
    out.reverse();
    Some(out)
}

fn slide2(
    tar_a: (usize, usize),
    tar_b: (usize, usize),
    fix: &mut [Vec<bool>],
    input: &Input,
    tiles: &mut [Vec<usize>],
    tree_tiles: &mut [Vec<usize>],
) -> Output {
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
    // eprintln!("前処理, empty:{:?}", empty);
    // for row in tiles.iter() {
    //     for t in row.iter() {
    //         eprint!("{:2} ", t);
    //     }
    //     eprintln!();
    // }
    // tar_a,tar_bとそれぞれの1マス隣の4マスからbを排除しなくていいパターンを列挙 6パターン
    if a_now == tar_a && b_now == tar_b {
        return out;
    }
    if tree_tiles[tar_a.0][tar_a.1] != tree_tiles[tar_b.0][tar_b.1] {
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
                    for _ in 0..(empty.1 + 2 - input.n) {
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
                    for _ in 0..(empty.0 + 2 - input.n) {
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

                    if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        && empty != (tar_b.0, tar_b.1 + 1)
                    {
                        // ax <- xがemptyでない
                        // .b
                        // とき
                        // a.
                        // b. にして回す 必勝パターン
                        // まず空きマスをinput.n - 2の列に持ってくる
                        if empty.1 == input.n - 1 {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else {
                            // input.n - 2 >= empty.1
                            for _ in 0..(input.n - 2 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                        }
                        // tar_b.0 + 1の行まで持ってくる
                        for _ in 0..(empty.0 - tar_b.0 - 1) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                        // a.
                        // be
                        // になった回す
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
                    }

                    if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        && empty != tar_b
                    {
                        // xb <- xがemptyでない
                        // a.
                        // とき
                        // .b
                        // .a にして回す 必勝パターン
                        // 空きマスがaの右隣に持って行きたい
                        if empty != (tar_b.0 + 1, tar_b.1 + 1) {
                            // まずは行をtar_b.0 + 2にする
                            if tar_b.0 + 2 > empty.0 {
                                for _ in 0..(tar_b.0 + 2 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            } else {
                                for _ in 0..(empty.0 - tar_b.0 - 2) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            }
                            // 列をinput.n - 1にする
                            for _ in 0..(input.n - 1 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                            // aの右隣に持って行く
                            for _ in 0..(empty.0 - tar_b.0 - 1) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                        }
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
                        // .b
                        // ea になったので回す
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
                        return out;
                    }

                    if !(tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1])
                    {
                        // .a
                        // .b
                        // でない場合、これにする
                        // 7パターン
                        if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 1
                            // ab
                            // ..
                            // 空きマスをbの上に持って行く
                            for _ in 0..(input.n - 1 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                            for _ in 0..(empty.0 - tar_b.0 - 1) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
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
                        } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 2
                            // ..
                            // ba
                            // こいつだけ別枠にしない？回しにくいから
                            // bを排除して"このa"(now_aとは限らない)をtar_bにslide、
                            // その後tar_a,tar_bをfixしてbをtar_bにslide

                            // 空きマスがtar_b.0 と同じ行にあるかで場合分け
                            // あるなら
                            // e. 　.a
                            // ba をbe にできる これはgreen 2
                            if empty.0 == tar_b.0 {
                                if empty.1 == tar_b.1 {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                                // green 2. かつもう空きマスがbの隣
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            } else {
                                // bの左隣に空きマスを持ってくる
                                // 空きマスは列から揃える
                                if tar_b.1 - 1 < empty.1 {
                                    for _ in 0..(empty.1 - tar_b.1 + 1) {
                                        out.push('L');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                        tiles[empty.0][empty.1 - 1] = 16;
                                        empty.1 -= 1;
                                    }
                                } else {
                                    for _ in 0..(tar_b.1 - 1 - empty.1) {
                                        out.push('R');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                        tiles[empty.0][empty.1 + 1] = 16;
                                        empty.1 += 1;
                                    }
                                }
                                // 空きマスを行をtar_b.0 + 1にする
                                for _ in 0..(empty.0 - tar_b.0 - 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                                // 空きマスがbの左隣
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                                // ...
                                // bea
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
                                // .a.
                                // be.
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
                        } else if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 3
                            // b.
                            // a
                            if empty == (tar_b.0, tar_b.1 + 1) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            } else if empty != (tar_b.0 + 1, tar_b.1 + 1) {
                                // まずは行をtar_b.0 + 2にする
                                if tar_b.0 + 2 > empty.0 {
                                    for _ in 0..(tar_b.0 + 2 - empty.0) {
                                        out.push('D');
                                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                        tiles[empty.0 + 1][empty.1] = 16;
                                        empty.0 += 1;
                                    }
                                } else {
                                    for _ in 0..(empty.0 - tar_b.0 - 2) {
                                        out.push('U');
                                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                        tiles[empty.0 - 1][empty.1] = 16;
                                        empty.0 -= 1;
                                    }
                                }
                                // 列をinput.n - 1にする
                                for _ in 0..(input.n - 1 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                                // aの隣まで持って行く
                                for _ in 0..(empty.0 - tar_b.0 - 1) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            }
                            // 左回り 右回りより1手減る
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
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 1
                            // ae <- eは必ずempty
                            // .b
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 2
                            // xa <- xはemptyでない
                            // b.
                            // まずは行をtar_b.0 + 2にする
                            if tar_b.0 + 2 > empty.0 {
                                for _ in 0..(tar_b.0 + 2 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            } else {
                                for _ in 0..(empty.0 - tar_b.0 - 2) {
                                    out.push('U');
                                    tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                    tiles[empty.0 - 1][empty.1] = 16;
                                    empty.0 -= 1;
                                }
                            }
                            // 列をinput.n - 1にする
                            for _ in 0..(input.n - 1 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                            // bの右隣に持って行く
                            for _ in 0..(empty.0 - tar_b.0 - 1) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                            // xa
                            // be
                            // 回す
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 3
                            // bx <- xはemptyでない
                            // .a
                            // まず空きマスをinput.n - 2の列に持ってくる
                            if empty.1 == input.n - 1 {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            } else {
                                // input.n - 2 >= empty.1
                                for _ in 0..(input.n - 2 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            }
                            // tar_b.0 + 1の行まで持ってくる
                            for _ in 0..(tar_b.0 + 1 - empty.0) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                            // bx
                            // ea
                            // 回す
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
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        } else if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 4
                            // eb <- eは必ずempty
                            // a.
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
                        }
                    }

                    if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                    {
                        // .a
                        // .b
                        // bを1回上にあげてからaをtar_bに（now_aをこの右上のaにする）
                        // tar_aとtar_bをfixしてnow_bを計算し、tar_bにbを移動すれば完成

                        // bの下に空きマスを持ってくる
                        // まずは行をtar_b.0 + 2にする
                        if tar_b.0 + 2 > empty.0 {
                            for _ in 0..(tar_b.0 + 2 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                        } else {
                            for _ in 0..(empty.0 - tar_b.0 - 2) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                        }
                        // 列をinput.n - 1にする
                        for _ in 0..(input.n - 1 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        // .a
                        // .b
                        // .e になった
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
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
                        // a.
                        // .e
                        // .b
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
                    unreachable!();
                } else {
                    // 下三角の場合
                    if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        && empty != (tar_b.0 + 1, tar_b.1)
                    {
                        // a.
                        // xb <- xがemptyでない
                        // とき
                        // ab
                        // .. にして回す 必勝パターン
                        // まず空きマスをinput.n - 2の行に持ってくる
                        if empty.0 == input.n - 1 {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else {
                            // input.n - 2 >= empty.0
                            for _ in 0..(input.n - 2 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                        }
                        // tar_b.1 + 1の列まで持ってくる
                        for _ in 0..(empty.1 - tar_b.1 - 1) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                        // ab
                        // .e
                        // になった回す
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
                    }

                    if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        && empty != tar_b
                    {
                        // xa <- xがemptyでない
                        // b.
                        // とき
                        // ..
                        // ba にして回す 必勝パターン
                        // 空きマスをaの下に持って行きたい
                        if empty != (tar_b.0 + 1, tar_b.1 + 1) {
                            // まずは列をtar_b.1 + 2にする
                            if tar_b.1 + 2 > empty.1 {
                                for _ in 0..(tar_b.1 + 2 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            } else {
                                for _ in 0..(empty.1 - tar_b.1 - 2) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            }
                            // 行をinput.n - 1にする
                            for _ in 0..(input.n - 1 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                            // aの下に持って行く
                            for _ in 0..(empty.1 - tar_b.1 - 1) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                        }
                        out.push('U');
                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                        tiles[empty.0 - 1][empty.1] = 16;
                        empty.0 -= 1;
                        // .e
                        // ba になったので回す
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
                        return out;
                    }

                    if !(tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1])
                    {
                        // ..
                        // ab
                        // でない場合、これにする
                        // 7パターン
                        if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 1
                            // a.
                            // b.
                            // 空きマスをbの右に持って行く
                            for _ in 0..(input.n - 1 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                            for _ in 0..(empty.1 - tar_b.1 - 1) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
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
                        } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 2
                            // .b
                            // .a
                            // こいつだけ別枠にしない？回しにくいから
                            // bを排除して"このa"(now_aとは限らない)をtar_bにslide、
                            // その後tar_a,tar_bをfixしてbをtar_bにslide

                            // 空きマスがtar_b.1 と同じ列にあるかで場合分け
                            // あるなら
                            // eb 　.b
                            // .a をae にできる これはgreen 4
                            if empty.1 == tar_b.1 {
                                if empty.0 == tar_b.0 {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                                // green 4. かつもう空きマスがbの下
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            } else {
                                // bの上に空きマスを持ってくる
                                // 空きマスは行から揃える
                                if tar_b.0 - 1 < empty.0 {
                                    for _ in 0..(empty.0 - tar_b.0 + 1) {
                                        out.push('U');
                                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                        tiles[empty.0 - 1][empty.1] = 16;
                                        empty.0 -= 1;
                                    }
                                } else {
                                    for _ in 0..(tar_b.0 - 1 - empty.0) {
                                        out.push('D');
                                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                        tiles[empty.0 + 1][empty.1] = 16;
                                        empty.0 += 1;
                                    }
                                }
                                // 空きマスの列をtar_b.1 + 1にする
                                for _ in 0..(empty.1 - tar_b.1 - 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                                // 空きマスがbの上
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                                // .b.
                                // .e.
                                // .a.
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
                                // .b.
                                // ae.
                                // ...
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
                        } else if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // orange 3
                            // ba
                            // ..
                            if empty == (tar_b.0 + 1, tar_b.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            } else if empty != (tar_b.0 + 1, tar_b.1 + 1) {
                                // まずは列をtar_b.1 + 2にする
                                if tar_b.1 + 2 > empty.1 {
                                    for _ in 0..(tar_b.1 + 2 - empty.1) {
                                        out.push('R');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                        tiles[empty.0][empty.1 + 1] = 16;
                                        empty.1 += 1;
                                    }
                                } else {
                                    for _ in 0..(empty.1 - tar_b.1 - 2) {
                                        out.push('L');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                        tiles[empty.0][empty.1 - 1] = 16;
                                        empty.1 -= 1;
                                    }
                                }
                                // 行をinput.n - 1にする
                                for _ in 0..(input.n - 1 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                                // aの下まで持って行く
                                for _ in 0..(empty.1 - tar_b.1 - 1) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            }
                            // 左回り 右回りより1手減る
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
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else if tiles[tar_b.0][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 1
                            // a.
                            // eb <- eは必ずempty
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 2
                            // xb <- xはemptyでない
                            // a.
                            // まずは列をtar_b.1 + 2にする
                            if tar_b.1 + 2 > empty.1 {
                                for _ in 0..(tar_b.1 + 2 - empty.1) {
                                    out.push('R');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                    tiles[empty.0][empty.1 + 1] = 16;
                                    empty.1 += 1;
                                }
                            } else {
                                for _ in 0..(empty.1 - tar_b.1 - 2) {
                                    out.push('L');
                                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                    tiles[empty.0][empty.1 - 1] = 16;
                                    empty.1 -= 1;
                                }
                            }
                            // 行をinput.n - 1にする
                            for _ in 0..(input.n - 1 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                            // bの下に持って行く
                            for _ in 0..(empty.1 - tar_b.1 - 1) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                            // xb
                            // ae
                            // 回す
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else if tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 3
                            // b.
                            // xa <- xはemptyでない
                            // まず空きマスをinput.n - 2の行に持ってくる
                            if empty.1 == input.n - 1 {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            } else {
                                // input.n - 2 >= empty.0
                                for _ in 0..(input.n - 2 - empty.0) {
                                    out.push('D');
                                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                    tiles[empty.0 + 1][empty.1] = 16;
                                    empty.0 += 1;
                                }
                            }
                            // tar_b.1 + 1の列まで持ってくる
                            for _ in 0..(tar_b.1 + 1 - empty.1) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                            // be
                            // .a
                            // 回す
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
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        } else if tiles[tar_b.0][tar_b.1 + 1] == tree_tiles[tar_a.0][tar_a.1]
                            && tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_b.0][tar_b.1]
                        {
                            // green 4
                            // ea <- eは必ずempty
                            // b.
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
                        }
                    }

                    if tiles[tar_b.0 + 1][tar_b.1] == tree_tiles[tar_a.0][tar_a.1]
                        && tiles[tar_b.0 + 1][tar_b.1 + 1] == tree_tiles[tar_b.0][tar_b.1]
                    {
                        // ..
                        // ab
                        // bを1回右にずらしてからaをtar_bに（now_aをこの左下のaにする）
                        // tar_aとtar_bをfixしてnow_bを計算し、tar_bにbを移動すれば完成

                        // bの右に空きマスを持ってくる
                        // まずは列をtar_b.1 + 2にする
                        if tar_b.1 + 2 > empty.1 {
                            for _ in 0..(tar_b.1 + 2 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                        } else {
                            for _ in 0..(empty.1 - tar_b.1 - 2) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                        }
                        // 行をinput.n - 1にする
                        for _ in 0..(input.n - 1 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        // ...
                        // ...
                        // abe になった
                        out.push('L');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                        tiles[empty.0][empty.1 - 1] = 16;
                        empty.1 -= 1;
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
                        // ...
                        // a..
                        // .eb
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
                    unreachable!();
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
                                    for _ in 0..(empty.0 - tar_b.0 - 1) {
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
                        fix[tar_b.0][tar_b.1] = true;
                        fix[tar_a.0][tar_a.1] = true;
                        a_now = get_now(tar_a, fix, input, tiles, tree_tiles);
                        fix[tar_b.0][tar_b.1] = false;
                        fix[tar_a.0][tar_a.1] = false;
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
                                    if a_now.0 < empty.0 {
                                        for _ in 0..(empty.0 - a_now.0 - 1) {
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
                                        out.push('D');
                                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                        tiles[empty.0 + 1][empty.1] = 16;
                                        empty.0 += 1;
                                    } else if empty.0 != input.n - 1 {
                                        out.push('D');
                                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                        tiles[empty.0 + 1][empty.1] = 16;
                                        empty.0 += 1;
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
                                    } else {
                                        out.push('U');
                                        tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                        tiles[empty.0 - 1][empty.1] = 16;
                                        empty.0 -= 1;
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
                                if a_now.0 < input.n - 1 {
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
                        }
                        a_now.1 = input.n - 1;
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
                            for _ in 0..(a_now.0 - tar_b.0 - 2) {
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
                            for _ in 0..(empty.1 - input.n + 2) {
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
                                    for _ in 0..(empty.1 - tar_b.1 - 1) {
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
                        fix[tar_b.0][tar_b.1] = true;
                        fix[tar_a.0][tar_a.1] = true;
                        a_now = get_now(tar_a, fix, input, tiles, tree_tiles);
                        fix[tar_b.0][tar_b.1] = false;
                        fix[tar_a.0][tar_a.1] = false;
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
                                    // a_now.0 > empty.0
                                    if a_now.1 < empty.1 {
                                        for _ in 0..(empty.1 - a_now.1 - 1) {
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
                                        out.push('R');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                        tiles[empty.0][empty.1 + 1] = 16;
                                        empty.1 += 1;
                                    } else if empty.1 != input.n - 1 {
                                        out.push('R');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                        tiles[empty.0][empty.1 + 1] = 16;
                                        empty.1 += 1;
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
                                    } else {
                                        out.push('L');
                                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                        tiles[empty.0][empty.1 - 1] = 16;
                                        empty.1 -= 1;
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
                                }
                            }
                            // 空きマスはa_nowと同じ列であって、a_now.0 <= empty.0となっている
                            for _ in 0..(empty.0 - a_now.0 - 1) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                            // 空きマスがa_nowの下に来たので、aを一番下の行で持って行く
                            for _ in 0..(input.n - 1 - a_now.0 - 1) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                                if a_now.1 < input.n - 1 {
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
                        }
                        a_now.0 = input.n - 1;
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
                            for _ in 0..(a_now.1 - tar_b.1 - 2) {
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
                            for _ in 0..(empty.0 - input.n + 2) {
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
                unreachable!();
            }
        }
        // チェック終わり
    } else {
        // aとbが同じ数字
        let mut count = 0;
        for i in 0..2 {
            for j in 0..2 {
                if tiles[tar_b.0 + i][tar_b.1 + j] == tree_tiles[tar_b.0][tar_b.1] {
                    count += 1;
                }
            }
        }
        if 2 <= count {
            let a = tree_tiles[tar_b.0][tar_b.1];
            if tar_a.0 <= tar_a.1 {
                // 上三角のとき
                // xa <- xはemptyでない
                // a.
                // や
                // ax
                // .a
                // の場合ここでreturnしない
                if tiles[tar_b.0][tar_b.1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a
                    || tiles[tar_b.0][tar_b.1 + 1] == a && tiles[tar_b.0 + 1][tar_b.1] == a
                {
                    if empty.0 == tar_b.0 {
                        // 1つ下げるだけ
                        out.push('D');
                        tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                        tiles[empty.0 + 1][empty.1] = 16;
                        empty.0 += 1;
                        return out;
                    }
                } else if tiles[tar_b.0 + 1][tar_b.1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a {
                    if empty.0 == tar_b.0 {
                        // e.    .e
                        // aa or aa
                        if empty.1 == tar_b.1 {
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
                        } else {
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
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                        return out;
                    }
                } else if tiles[tar_b.0][tar_b.1 + 1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a {
                    // .a
                    // .a
                    // まずinput.n - 2の列に持って行く
                    if empty.1 < input.n - 2 {
                        for _ in 0..(input.n - 2 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                    } else {
                        for _ in 0..(empty.1 + 2 - input.n) {
                            out.push('L');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                            tiles[empty.0][empty.1 - 1] = 16;
                            empty.1 -= 1;
                        }
                    }
                    // aの隣に持って行く
                    for _ in 0..(empty.0 - tar_b.0) {
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
                } else if tiles[tar_b.0][tar_b.1] == a && tiles[tar_b.0 + 1][tar_b.1] == a {
                    // a.
                    // a.
                    // input.n - 1列じゃなければ列を合わせたい
                    if empty.1 != input.n - 1 {
                        // まずtar_b.0 + 2の行に持って行く
                        if empty.0 < tar_b.0 + 2 {
                            for _ in 0..(tar_b.0 + 2 - empty.0) {
                                out.push('D');
                                tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                                tiles[empty.0 + 1][empty.1] = 16;
                                empty.0 += 1;
                            }
                        } else {
                            for _ in 0..(empty.0 - tar_b.0 - 2) {
                                out.push('U');
                                tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                                tiles[empty.0 - 1][empty.1] = 16;
                                empty.0 -= 1;
                            }
                        }
                        for _ in 0..(input.n - 1 - empty.1) {
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                    }
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
                    out.push('D');
                    tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                    tiles[empty.0 + 1][empty.1] = 16;
                    empty.0 += 1;
                    return out;
                }
            } else {
                // 下三角のとき
                if tiles[tar_b.0][tar_b.1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a
                    || tiles[tar_b.0][tar_b.1 + 1] == a && tiles[tar_b.0 + 1][tar_b.1] == a
                {
                    if empty.1 == tar_b.1 {
                        // 1つ右
                        out.push('R');
                        tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                        tiles[empty.0][empty.1 + 1] = 16;
                        empty.1 += 1;
                        return out;
                    }
                } else if tiles[tar_b.0][tar_b.1 + 1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a {
                    if empty.1 == tar_b.1 {
                        // ea    .a
                        // .a or ea
                        if empty.0 == tar_b.0 {
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
                        } else {
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
                            out.push('R');
                            tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                            tiles[empty.0][empty.1 + 1] = 16;
                            empty.1 += 1;
                        }
                        return out;
                    }
                } else if tiles[tar_b.0 + 1][tar_b.1] == a && tiles[tar_b.0 + 1][tar_b.1 + 1] == a {
                    // ..
                    // aa
                    // まずinput.n - 2の行に持って行く
                    if empty.1 < input.n - 2 {
                        for _ in 0..(input.n - 2 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                    } else {
                        for _ in 0..(empty.0 + 2 - input.n) {
                            out.push('U');
                            tiles[empty.0][empty.1] = tiles[empty.0 - 1][empty.1];
                            tiles[empty.0 - 1][empty.1] = 16;
                            empty.0 -= 1;
                        }
                    }
                    // aの上に持って行く
                    for _ in 0..(empty.1 - tar_b.1) {
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
                } else if tiles[tar_b.0][tar_b.1] == a && tiles[tar_b.0][tar_b.1 + 1] == a {
                    // aa
                    // ..
                    // input.n - 1行じゃなければ列を合わせたい
                    if empty.0 != input.n - 1 {
                        // まずtar_b.1 + 2の列に持って行く
                        if empty.1 < tar_b.1 + 2 {
                            for _ in 0..(tar_b.1 + 2 - empty.1) {
                                out.push('R');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                                tiles[empty.0][empty.1 + 1] = 16;
                                empty.1 += 1;
                            }
                        } else {
                            for _ in 0..(empty.1 - tar_b.1 - 2) {
                                out.push('L');
                                tiles[empty.0][empty.1] = tiles[empty.0][empty.1 - 1];
                                tiles[empty.0][empty.1 - 1] = 16;
                                empty.1 -= 1;
                            }
                        }
                        for _ in 0..(input.n - 1 - empty.0) {
                            out.push('D');
                            tiles[empty.0][empty.1] = tiles[empty.0 + 1][empty.1];
                            tiles[empty.0 + 1][empty.1] = 16;
                            empty.0 += 1;
                        }
                    }
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
                    out.push('R');
                    tiles[empty.0][empty.1] = tiles[empty.0][empty.1 + 1];
                    tiles[empty.0][empty.1 + 1] = 16;
                    empty.1 += 1;
                    return out;
                }
            }
        }
    }
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
) -> Output {
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
    // eprintln!("tar:{:?}, now:{:?}, empty:{:?}", tar, now, empty);
    // for row in tiles.iter() {
    //     for t in row.iter() {
    //         eprint!("{:2} ", t);
    //     }
    //     eprintln!();
    // }

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
                    for _ in 0..(empty.0 - now.0 + 1) {
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
    fails: &mut [Vec<usize>],
    last_tiles: &mut [Vec<usize>],
    timer: &Timer,
) -> bool {
    if *count >= 2_250_000 {
        return false;
    }
    if *count % 100 == 0 && TIMELIMIT < timer.get_time() {
        let last_tile_count = {
            let mut count = vec![0; 17];
            for row in last_tiles.iter() {
                for t in row.iter() {
                    count[*t] += 1;
                }
            }
            count
        };
        let mut in_tile_count = {
            let mut count = vec![0; 17];
            for row in input.tiles.iter() {
                for t in row.iter() {
                    count[*t] += 1;
                }
            }
            count
        };
        for i in 1..16 {
            in_tile_count[i] -= last_tile_count[i];
        }
        for i in 0..input.n {
            for j in 0..input.n {
                if last_tiles[i][j] == 0 {
                    for tile_i in 1..16 {
                        if in_tile_count[tile_i] > 0 {
                            last_tiles[i][j] = tile_i;
                            in_tile_count[tile_i] -= 1;
                            break;
                        }
                    }
                }
            }
        }
        if let Some(out) = construct(input, last_tiles) {
            println!("{}", out.iter().join(""));
        } else {
            println!();
            for _ in 0..100 {}
            eprintln!("作れないやつ");
        }
        eprintln!("uo---");
        exit(0);
    }
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
        let mut connect_count = 0;
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
                    } else {
                        connect_count += 1;
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
                if 1 < connect_count && find_cycle(pos, input, now_tiles) {
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                    continue;
                }
                if tile_count.iter().skip(1).sum::<i32>() == 0 && !find_cycle(pos, input, now_tiles)
                {
                    let mut tree_tiles = vec![vec![0; input.n]; input.n];
                    for (i, row) in now_tiles.iter().enumerate() {
                        for (j, t) in row.iter().enumerate() {
                            tree_tiles[i][j] = *t;
                        }
                    }
                    // eprintln!("count: {}", count);
                    // for row in now_tiles.iter() {
                    //     for t in row.iter() {
                    //         eprint!("{:x}", t);
                    //     }
                    //     eprintln!();
                    // }
                    // eprintln!();
                    // for row in now_tiles.iter() {
                    //     for t in row.iter() {
                    //         eprint!("{:2} ", t);
                    //     }
                    //     eprintln!();
                    // }
                    if let Some(out) = construct(input, &mut tree_tiles) {
                        println!("{}", out.iter().take(input.t).join(""));
                        return true;
                    }
                    // parity checkに失敗したらNoneが返る
                }
                if (can_empty_space || open_count == 0 && can_empty_space2)
                    && is_empty_space(input, now_tiles)
                {
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                    continue;
                }
                // if !is_connected(pos, input, now_tiles) {
                //     now_tiles[pos.0][pos.1] = 0;
                //     tile_count[tile_i] += 1;
                //     continue;
                // }
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
                        fails,
                        last_tiles,
                        timer,
                    ) {
                        return true;
                    }
                }
                now_tiles[pos.0][pos.1] = 0;
                tile_count[tile_i] += 1;
            } else {
                next_poses.extend_from_slice(&dij);
                now_tiles[pos.0][pos.1] = tile_i;
                tile_count[tile_i] -= 1;
                if (can_empty_space || open_count == 0 && can_empty_space2)
                    && is_empty_space(input, now_tiles)
                {
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                    for _ in 0..dij.len() {
                        next_poses.pop();
                    }
                    continue;
                }
                if 1 < connect_count && find_cycle(pos, input, now_tiles) {
                    now_tiles[pos.0][pos.1] = 0;
                    tile_count[tile_i] += 1;
                    for _ in 0..dij.len() {
                        next_poses.pop();
                    }
                    continue;
                }
                for &(i2, j2) in dij.iter() {
                    if dfs(
                        (i2, j2),
                        input,
                        tile_count,
                        now_tiles,
                        next_poses,
                        tile_is,
                        count,
                        fails,
                        last_tiles,
                        timer,
                    ) {
                        return true;
                    }
                }
                now_tiles[pos.0][pos.1] = 0;
                tile_count[tile_i] += 1;
                for _ in 0..dij.len() {
                    next_poses.pop();
                }
            }
        }
    }
    *count += 1;
    // if *count <= 1000000 && *count % 100000 == 0 {
    // eprintln!("count: {}", count);
    // eprintln!("{:?}", tile_count);
    // eprintln!("{} {}", input.n, input.t);
    // for row in now_tiles.iter() {
    //     for t in row.iter() {
    //         if *t == 16 {
    //             eprint!("{:x}", 0);
    //         } else {
    //             eprint!("{:x}", t);
    //         }
    //     }
    //     eprintln!();
    // }
    // }
    fails[pos.0][pos.1] += 1;
    if 1_000_000 == *count {
        for i in 0..input.n {
            for j in 0..input.n {
                last_tiles[i][j] = now_tiles[i][j];
            }
        }
        // eprintln!("count: {}", count);
        // eprintln!("{:?}", tile_count);
        // eprintln!("{} {}", input.n, input.t);
        // for row in now_tiles.iter() {
        //     for t in row.iter() {
        //         if *t == 16 {
        //             eprint!("{:x}", 0);
        //         } else {
        //             eprint!("{:x}", t);
        //         }
        //     }
        //     eprintln!();
        // }
    }
    false
}

#[allow(dead_code)]
fn is_connected(pos: (usize, usize), input: &Input, tiles: &[Vec<usize>]) -> bool {
    // posからBFSして今置いてあるタイルに全部行けるかチェック
    let mut que = VecDeque::new();
    let mut visited = vec![vec![false; input.n]; input.n];
    visited[pos.0][pos.1] = true;
    que.push_back(pos);
    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        for (d, (di, dj)) in DIJ.iter().enumerate() {
            let ni = v.0 + *di;
            let nj = v.1 + *dj;
            if ni >= input.n || nj >= input.n {
                continue;
            }
            if tiles[v.0][v.1] != 0 && tiles[v.0][v.1] != 16 && tiles[v.0][v.1] & (1 << d) == 0 {
                // tileが開いてない方向には進めない
                continue;
            }
            if visited[ni][nj] {
                continue;
            }
            if tiles[ni][nj] != 0 && tiles[ni][nj] != 16 && tiles[ni][nj] & (1 << (d ^ 2)) == 0 {
                // 空きマスでなくてタイルが開いてないなら進んではいけない
                continue;
            }
            visited[ni][nj] = true;
            que.push_back((ni, nj));
        }
    }
    for i in 0..input.n {
        for j in 0..input.n {
            if tiles[i][j] != 0 && tiles[i][j] != 16 && !visited[i][j] {
                return false;
            }
        }
    }
    true
}

fn find_cycle(pos: (usize, usize), input: &Input, tiles: &[Vec<usize>]) -> bool {
    // posからbfsして2回訪れる場所があればcycleあり
    let mut que = VecDeque::new();
    let mut visited = vec![vec![false; input.n]; input.n];
    visited[pos.0][pos.1] = true;
    que.push_back((pos, 4));
    while !que.is_empty() {
        let (v, prev_d) = que.pop_front().unwrap();
        for (d, (di, dj)) in DIJ.iter().enumerate() {
            let ni = v.0 + *di;
            let nj = v.1 + *dj;
            if ni >= input.n || nj >= input.n {
                continue;
            }
            if d ^ 2 == prev_d {
                // 来た方向に戻るのを禁止
                continue;
            }
            if tiles[v.0][v.1] & (1 << d) == 0 {
                // tileが開いてない方向には進めない
                continue;
            }
            if visited[ni][nj] {
                // 閉路あり
                return true;
            }
            visited[ni][nj] = true;
            que.push_back(((ni, nj), d));
        }
    }
    false
}

fn parity_check(input: &Input, tiles: &[usize], tree_tiles: &[Vec<usize>]) -> bool {
    // 右下にめっちゃ同じ数あったら大体成功しそう
    let mut set = HashSet::new();
    for &t in tiles.iter() {
        set.insert(t);
    }
    if set.len() < 9 {
        return true;
    }

    let mut count = 0;
    let mut map = HashMap::new();
    for i in input.n - 3..input.n {
        for j in input.n - 3..input.n {
            let e = if tree_tiles[i][j] != 16 {
                map.entry(tree_tiles[i][j]).or_insert(0)
            } else {
                map.entry(0).or_insert(0)
            };
            if *e == 0 {
                *e = (i + 3 - input.n) * 3 + (j + 3 - input.n);
            }
        }
    }

    for i in 0..3 * 3 {
        for j in 0..3 * 3 {
            if i < j && map[&tiles[i]] > map[&tiles[j]] {
                count += 1;
            }
        }
    }
    let mut pos1 = (0, 0);
    for i in input.n - 3..input.n {
        for j in input.n - 3..input.n {
            if tree_tiles[i][j] == 16 {
                pos1.0 = i + 3 - input.n;
                pos1.1 = j + 3 - input.n;
            }
        }
    }
    let mut pos2 = (0, 0);
    for i in 0..3 {
        for j in 0..3 {
            if tiles[i * 3 + j] == 0 {
                pos2.0 = i;
                pos2.1 = j;
            }
        }
    }
    let dist = (pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs();
    count % 2 == dist % 2
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

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }

    // fn reset(&mut self) {
    //     self.start_time = get_time();
    // }
}
