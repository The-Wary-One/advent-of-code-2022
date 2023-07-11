use std::{collections::HashSet, str::Lines};

mod forest;
use forest::*;

struct ForestParser<'a, 'b>
where
    'a: 'b,
{
    lines: &'b mut Lines<'a>,
}

impl<'a, 'b> ForestParser<'a, 'b>
where
    'a: 'b,
{
    fn new(lines: &'b mut Lines<'a>) -> Self {
        Self { lines }
    }

    fn parse(self) -> Forest {
        let grid = self
            .lines
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let height = c.to_string().parse().expect("safe");
                        Tree::new(height, (x, y))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Forest::new(grid)
    }
}

fn get_forest_and_visible_trees(mut input: Lines) -> (Forest, HashSet<Tree>) {
    let parser = ForestParser::new(&mut input);
    let forest = parser.parse();
    let visible_trees = forest
        // Iterate over all rows and columns.
        .iter_rows_then_columns()
        .fold(HashSet::<Tree>::new(), |mut set, line| {
            let mut highest_tree = None;
            line.iter().for_each(|tree| {
                if highest_tree.is_some_and(|h| tree.height() > h) || highest_tree.is_none() {
                    set.insert(tree.clone());
                    highest_tree = Some(tree.height())
                }
            });
            highest_tree = None;
            line.iter().rev().for_each(|tree| {
                if highest_tree.is_some_and(|h| tree.height() > h) || highest_tree.is_none() {
                    set.insert(tree.clone());
                    highest_tree = Some(tree.height())
                }
            });
            set
        });
    (forest, visible_trees)
}

pub fn solve_part1(input: Lines) -> usize {
    let (_, visible_tree_set) = get_forest_and_visible_trees(input);
    visible_tree_set.len()
}

pub fn solve_part2(input: Lines) -> usize {
    let (forest, visible_tree_set) = get_forest_and_visible_trees(input);
    visible_tree_set
        .iter()
        .map(|tree| forest.tree_scenic_score(tree))
        .max()
        .expect("safe")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT.lines()), 8);
    }
}
