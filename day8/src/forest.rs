use std::str::FromStr;

use thiserror::Error;

pub(crate) struct Forest {
    grid: Vec<Vec<Tree>>,
}

impl Forest {
    pub(crate) fn new(grid: Vec<Vec<Tree>>) -> Self {
        Forest { grid }
    }

    pub(crate) fn iter_rows_then_columns(&self) -> RowsThenColumnIterator {
        RowsThenColumnIterator::new(self)
    }
}

pub(crate) struct RowsThenColumnIterator<'a> {
    forest: &'a Forest,
    row_len: usize,
    current_line: usize,
}

impl<'a> RowsThenColumnIterator<'a> {
    fn new(forest: &'a Forest) -> Self {
        Self {
            forest,
            row_len: forest.grid.len(),
            current_line: 0,
        }
    }
}

impl<'a> Iterator for RowsThenColumnIterator<'a> {
    type Item = Vec<Tree>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.current_line;
        self.current_line += 1;
        if line < self.row_len {
            // Return a row.
            self.forest.grid.get(line).cloned()
        } else {
            // Build and return a col.
            let col_nb = line - self.row_len;
            self.forest.grid.iter().fold(Some(Vec::new()), |acc, row| {
                acc.and_then(|mut v| {
                    row.get(col_nb).cloned().map(|t| {
                        v.push(t);
                        v
                    })
                })
            })
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) struct Tree {
    height: TreeHeight,
    position: (usize, usize),
}

impl Tree {
    pub(crate) fn new(height: TreeHeight, position: (usize, usize)) -> Self {
        Self { height, position }
    }

    pub(crate) fn height(&self) -> TreeHeight {
        self.height
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub(crate) struct TreeHeight(u8);

impl TreeHeight {
    const MAX: u8 = 9;
}

#[derive(Error, Debug)]
pub enum TryFromTreeHeightError {
    #[error("{0} is higher than the max tree height (i.e. {})", TreeHeight::MAX)]
    TooBig(u8),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl TryFrom<u8> for TreeHeight {
    type Error = TryFromTreeHeightError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 10 {
            Ok(TreeHeight(value))
        } else {
            Err(TryFromTreeHeightError::TooBig(value))
        }
    }
}

impl FromStr for TreeHeight {
    type Err = TryFromTreeHeightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = s.parse::<u8>()?;
        u.try_into()
    }
}
