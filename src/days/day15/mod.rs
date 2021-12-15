//! # Day 15: Chiton
//!
//! You've almost reached the exit of the cave, but the walls are getting closer together. Your submarine can barely still fit, though; the main problem is that the walls of the cave are covered in chitons, and it would be best not to bump any of them.
//!
//! The cavern is large, but has a very low ceiling, restricting your motion to two dimensions. The shape of the cavern resembles a square; a quick scan of chiton density produces a map of risk level throughout the cave (your puzzle input). For example:
//!
//! 1163751742
//! 1381373672
//! 2136511328
//! 3694931569
//! 7463417111
//! 1319128137
//! 1359912421
//! 3125421639
//! 1293138521
//! 2311944581
//!
//! You start in the top left position, your destination is the bottom right position, and you cannot move diagonally. The number at each position is its risk level; to determine the total risk of an entire path, add up the risk levels of each position you enter (that is, don't count the risk level of your starting position unless you enter it; leaving it adds no risk to your total).
//!
//! Your goal is to find a path with the lowest total risk. In this example, a path with the lowest total risk is highlighted here:
//!
//! 1163751742
//! 1381373672
//! 2136511328
//! 3694931569
//! 7463417111
//! 1319128137
//! 1359912421
//! 3125421639
//! 1293138521
//! 2311944581
//!
//! The total risk of this path is 40 (the starting position is never entered, so its risk is not counted).
//!
//! What is the lowest total risk of any path from the top left to the bottom right?
//!
//! Your puzzle answer was 621.
//!
//! ## Part Two
//!
//! Now that you know how to find low-risk paths in the cave, you can try to find your way out.
//!
//! The entire cave is actually five times larger in both dimensions than you thought; the area you originally scanned is just one tile in a 5x5 tile area that forms the full map. Your original map tile repeats to the right and downward; each time the tile repeats to the right or downward, all of its risk levels are 1 higher than the tile immediately up or left of it. However, risk levels above 9 wrap back around to 1. So, if your original map had some position with a risk level of 8, then that same position on each of the 25 total tiles would be as follows:
//!
//! 8 9 1 2 3
//! 9 1 2 3 4
//! 1 2 3 4 5
//! 2 3 4 5 6
//! 3 4 5 6 7
//! Each single digit above corresponds to the example position with a value of 8 on the top-left tile. Because the full map is actually five times larger in both dimensions, that position appears a total of 25 times, once in each duplicated tile, with the values shown above.
//!
//! Here is the full five-times-as-large version of the first example above, with the original map in the top left corner highlighted:
//!
//! 11637517422274862853338597396444961841755517295286
//! 13813736722492484783351359589446246169155735727126
//! 21365113283247622439435873354154698446526571955763
//! 36949315694715142671582625378269373648937148475914
//! 74634171118574528222968563933317967414442817852555
//! 13191281372421239248353234135946434524615754563572
//! 13599124212461123532357223464346833457545794456865
//! 31254216394236532741534764385264587549637569865174
//! 12931385212314249632342535174345364628545647573965
//! 23119445813422155692453326671356443778246755488935
//! 22748628533385973964449618417555172952866628316397
//! 24924847833513595894462461691557357271266846838237
//! 32476224394358733541546984465265719557637682166874
//! 47151426715826253782693736489371484759148259586125
//! 85745282229685639333179674144428178525553928963666
//! 24212392483532341359464345246157545635726865674683
//! 24611235323572234643468334575457944568656815567976
//! 42365327415347643852645875496375698651748671976285
//! 23142496323425351743453646285456475739656758684176
//! 34221556924533266713564437782467554889357866599146
//! 33859739644496184175551729528666283163977739427418
//! 35135958944624616915573572712668468382377957949348
//! 43587335415469844652657195576376821668748793277985
//! 58262537826937364893714847591482595861259361697236
//! 96856393331796741444281785255539289636664139174777
//! 35323413594643452461575456357268656746837976785794
//! 35722346434683345754579445686568155679767926678187
//! 53476438526458754963756986517486719762859782187396
//! 34253517434536462854564757396567586841767869795287
//! 45332667135644377824675548893578665991468977611257
//! 44961841755517295286662831639777394274188841538529
//! 46246169155735727126684683823779579493488168151459
//! 54698446526571955763768216687487932779859814388196
//! 69373648937148475914825958612593616972361472718347
//! 17967414442817852555392896366641391747775241285888
//! 46434524615754563572686567468379767857948187896815
//! 46833457545794456865681556797679266781878137789298
//! 64587549637569865174867197628597821873961893298417
//! 45364628545647573965675868417678697952878971816398
//! 56443778246755488935786659914689776112579188722368
//! 55172952866628316397773942741888415385299952649631
//! 57357271266846838237795794934881681514599279262561
//! 65719557637682166874879327798598143881961925499217
//! 71484759148259586125936169723614727183472583829458
//! 28178525553928963666413917477752412858886352396999
//! 57545635726865674683797678579481878968159298917926
//! 57944568656815567976792667818781377892989248891319
//! 75698651748671976285978218739618932984172914319528
//! 56475739656758684176786979528789718163989182927419
//! 67554889357866599146897761125791887223681299833479
//!
//! Equipped with the full map, you can now find a path from the top left corner to the bottom right corner with the lowest total risk:
//!
//! 11637517422274862853338597396444961841755517295286
//! 13813736722492484783351359589446246169155735727126
//! 21365113283247622439435873354154698446526571955763
//! 36949315694715142671582625378269373648937148475914
//! 74634171118574528222968563933317967414442817852555
//! 13191281372421239248353234135946434524615754563572
//! 13599124212461123532357223464346833457545794456865
//! 31254216394236532741534764385264587549637569865174
//! 12931385212314249632342535174345364628545647573965
//! 23119445813422155692453326671356443778246755488935
//! 22748628533385973964449618417555172952866628316397
//! 24924847833513595894462461691557357271266846838237
//! 32476224394358733541546984465265719557637682166874
//! 47151426715826253782693736489371484759148259586125
//! 85745282229685639333179674144428178525553928963666
//! 24212392483532341359464345246157545635726865674683
//! 24611235323572234643468334575457944568656815567976
//! 42365327415347643852645875496375698651748671976285
//! 23142496323425351743453646285456475739656758684176
//! 34221556924533266713564437782467554889357866599146
//! 33859739644496184175551729528666283163977739427418
//! 35135958944624616915573572712668468382377957949348
//! 43587335415469844652657195576376821668748793277985
//! 58262537826937364893714847591482595861259361697236
//! 96856393331796741444281785255539289636664139174777
//! 35323413594643452461575456357268656746837976785794
//! 35722346434683345754579445686568155679767926678187
//! 53476438526458754963756986517486719762859782187396
//! 34253517434536462854564757396567586841767869795287
//! 45332667135644377824675548893578665991468977611257
//! 44961841755517295286662831639777394274188841538529
//! 46246169155735727126684683823779579493488168151459
//! 54698446526571955763768216687487932779859814388196
//! 69373648937148475914825958612593616972361472718347
//! 17967414442817852555392896366641391747775241285888
//! 46434524615754563572686567468379767857948187896815
//! 46833457545794456865681556797679266781878137789298
//! 64587549637569865174867197628597821873961893298417
//! 45364628545647573965675868417678697952878971816398
//! 56443778246755488935786659914689776112579188722368
//! 55172952866628316397773942741888415385299952649631
//! 57357271266846838237795794934881681514599279262561
//! 65719557637682166874879327798598143881961925499217
//! 71484759148259586125936169723614727183472583829458
//! 28178525553928963666413917477752412858886352396999
//! 57545635726865674683797678579481878968159298917926
//! 57944568656815567976792667818781377892989248891319
//! 75698651748671976285978218739618932984172914319528
//! 56475739656758684176786979528789718163989182927419
//! 67554889357866599146897761125791887223681299833479
//!
//! The total risk of this path is 315 (the starting position is still never entered, so its risk is not counted).
//!
//! Using the full map, what is the lowest total risk of any path from the top left to the bottom right?
//!
//! Your puzzle answer was 2904.

use itertools::Itertools;
use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
};

use crate::{day::Challenge, parse_input_raw};

/// Day 15 implementation.
pub struct Day15;

struct Cave {
    array: Vec<Vec<u64>>,
    graph: DiGraph<u64, u64>,
}

impl Cave {
    pub fn get_lower_risk_path_sum(&self) -> u64 {
        let (cost, _) = self.get_min_path();
        cost
    }

    pub fn get_min_path(&self) -> (u64, Vec<NodeIndex>) {
        let target = NodeIndex::new(self.graph.node_count() - 1);
        algo::astar(
            &self.graph,
            0.into(),
            |x| x == target,
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap()
    }

    fn round_to_1(v: u64) -> u64 {
        let mut v = v;
        while v > 9 {
            v -= 9
        }
        v
    }

    pub fn create_full_map(&self) -> Self {
        let multiplicator = 5;
        let initial_width = self.array[0].len();
        let initial_height = self.array.len();
        let width = initial_width * multiplicator;
        let height = initial_height * multiplicator;

        let compute_offset = |x: usize, y: usize| -> u64 {
            let offset_x = (x as f32 / initial_width as f32) as u64;
            let offset_y = (y as f32 / initial_height as f32) as u64;
            offset_x + offset_y
        };

        let mut node_weights = vec![];
        let mut edge_weights = vec![];
        for y in 0..height {
            for x in 0..width {
                let idx = (x + y * width) as u32;
                let v = Self::round_to_1(
                    self.array[y % initial_height][x % initial_width] + compute_offset(x, y),
                );
                node_weights.push((idx, v));

                if x != width - 1 {
                    let other_v = Self::round_to_1(
                        self.array[y % initial_height][(x + 1) % initial_width]
                            + compute_offset(x + 1, y),
                    );
                    edge_weights.push((idx, idx + 1, other_v));
                    edge_weights.push((idx + 1, idx, v));
                }

                if y != height - 1 {
                    let other_v = Self::round_to_1(
                        self.array[(y + 1) % initial_height][x % initial_width]
                            + compute_offset(x, y + 1),
                    );
                    edge_weights.push((idx, idx + width as u32, other_v));
                    edge_weights.push((idx + width as u32, idx, v));
                }
            }
        }

        let mut graph = DiGraph::new();
        for (_, w) in node_weights {
            graph.add_node(w);
        }

        for (e1, e2, w) in edge_weights {
            graph.update_edge(e1.into(), e2.into(), w);
        }

        Self {
            array: self.array.clone(),
            graph,
        }
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        let array = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|x| x.to_digit(10).unwrap() as u64)
                    .collect_vec()
            })
            .collect_vec();
        let width = array[0].len();
        let height = array.len();

        let mut edges = vec![];
        for (y, l) in array.iter().enumerate() {
            for (x, &v) in l.iter().enumerate() {
                let idx = (x + y * width) as u32;

                if x != width - 1 {
                    let other_v = l[x + 1];
                    edges.push((idx, idx + 1, other_v));
                    edges.push((idx + 1, idx, v));
                }

                if y != height - 1 {
                    let other_v = array[y + 1][x];
                    edges.push((idx, idx + width as u32, other_v));
                    edges.push((idx + width as u32, idx, v));
                }
            }
        }

        Self {
            array,
            graph: DiGraph::from_edges(edges),
        }
    }
}

impl Challenge for Day15 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        let cave = Cave::from(parse_input_raw!());
        cave.get_lower_risk_path_sum().to_string()
    }

    fn run_ex2(&mut self) -> String {
        let cave = Cave::from(parse_input_raw!());
        let cave = cave.create_full_map();
        cave.get_lower_risk_path_sum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::Cave;

    create_day_tests!("15", "621", "2904");

    const SMALL_SAMPLE_DATA: &str = indoc::indoc! {"
        1163
        1381
        2136
        3694"
    };

    const SAMPLE_DATA: &str = indoc::indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581"
    };

    const SAMPLE_DATA_2: &str = indoc::indoc! {"
        1911191111
        1119111991
        9999999111
        9999911199
        9999119999
        9999199999
        9111199999
        9199999111
        9111911191
        9991119991"
    };

    #[test]
    fn test_small_sample() {
        let cave = Cave::from(SMALL_SAMPLE_DATA);
        assert_eq!(cave.get_lower_risk_path_sum(), 17);
    }

    #[test]
    fn test_sample() {
        let cave = Cave::from(SAMPLE_DATA);
        assert_eq!(cave.get_lower_risk_path_sum(), 40);
    }

    #[test]
    fn test_sample_2() {
        let cave = Cave::from(SAMPLE_DATA_2);
        assert_eq!(cave.get_lower_risk_path_sum(), 40);
    }

    #[test]
    fn test_sample_x5() {
        let cave = Cave::from(SAMPLE_DATA);
        let cave = cave.create_full_map();
        assert_eq!(cave.get_lower_risk_path_sum(), 315);
    }
}
