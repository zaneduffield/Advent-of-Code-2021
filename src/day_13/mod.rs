use std::{cmp::Ordering, collections::BTreeSet};

use itertools::Itertools;

type Coords = (usize, usize);
type Basin = BTreeSet<Coords>;

#[derive(Clone)]
struct Fold {
    is_1st_dim: bool,
    val: usize,
}

#[derive(Clone)]
struct Grid {
    values: BTreeSet<Coords>,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Grid {
    let mut folds = vec![];
    let mut values = BTreeSet::new();
    for line in input.lines().filter(|s| !s.is_empty()) {
        match line.strip_prefix("fold along ") {
            Some(fold) => {
                let (axis, val) = fold.split_once("=").unwrap();
                folds.push(Fold {
                    is_1st_dim: axis == "x",
                    val: val.parse().unwrap(),
                })
            }
            None => {
                let (x, y) = line.split_once(",").unwrap();
                values.insert((x.parse().unwrap(), y.parse().unwrap()));
            }
        }
    }

    folds.reverse();
    Grid { values, folds }
}

fn fold_grid(grid: &mut Grid) {
    let fold = match grid.folds.pop() {
        None => return,
        Some(v) => v,
    };

    let mut new_values = BTreeSet::new();
    grid.values.iter().for_each(|p| {
        let mut coord = if fold.is_1st_dim { p.0 } else { p.1 };
        new_values.insert(if coord > fold.val {
            coord = 2 * fold.val - coord;
            if fold.is_1st_dim {
                (coord, p.1)
            } else {
                (p.0, coord)
            }
        } else {
            *p
        });
    });
    grid.values = new_values;
}

#[aoc(day13, part1)]
fn part_1(grid: &Grid) -> usize {
    let mut grid = grid.to_owned();
    fold_grid(&mut grid);
    grid.values.len()
}

#[aoc(day13, part2)]
fn part_2(grid: &Grid) -> String {
    let mut grid = grid.to_owned();
    while !grid.folds.is_empty() {
        fold_grid(&mut grid);
    }
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(17, part_1(&parse(input)));
        assert_eq!("O", part_2(&parse(input)));
    }
}
