use std::cmp::Ordering;

use crate::graph::{AdjacencyList, Edge};

#[derive(Clone, Copy)]
pub enum Elevation {
    Start,
    End,
    Other(u8),
}

impl Elevation {
    pub fn elevation(&self) -> u8 {
        match self {
            Elevation::Start => 0,
            Elevation::End => 25,
            Elevation::Other(e) => *e,
        }
    }

    fn is_reachable(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal | Ordering::Greater => true,
            Ordering::Less => other.elevation() - self.elevation() == 1,
        }
    }
}

impl PartialOrd for Elevation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elevation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.elevation().cmp(&other.elevation())
    }
}

impl PartialEq for Elevation {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Elevation {}

impl TryFrom<char> for Elevation {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' => Ok(Elevation::Other(value as u8 - b'a')),
            'S' => Ok(Elevation::Start),
            'E' => Ok(Elevation::End),
            _ => Err("bad input"),
        }
    }
}

pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct HeightMap {
    inner: Box<[Elevation]>,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

impl HeightMap {
    pub fn find_shortest_path(&self) -> usize {
        let adj_list = self.build_adjacency_list();
        let shortest_path = adj_list.dijkstra_with_priority_queue(self.start, self.end);
        shortest_path.expect("safe").len()
    }

    fn position_from_raw(width: usize, idx: usize) -> Position {
        let x = idx % width;
        let y = idx / width;
        Position::new(x, y)
    }

    fn position_to_raw(width: usize, position: Position) -> usize {
        position.y() * width + position.x()
    }

    fn adjacent_no_diag(&self, current: &Position) -> [Option<Position>; 4] {
        let up = match current {
            Position { y: 0, .. } => None,
            Position { x, y } => Some(Position::new(*x, y - 1)),
        };
        let down = match current {
            Position { y, .. } if *y == self.height - 1 => None,
            Position { x, y } => Some(Position::new(*x, y + 1)),
        };
        let left = match current {
            Position { x: 0, .. } => None,
            Position { x, y } => Some(Position::new(x - 1, *y)),
        };
        let right = match current {
            Position { x, .. } if *x == self.width - 1 => None,
            Position { x, y } => Some(Position::new(x + 1, *y)),
        };
        [up, right, down, left]
    }

    fn build_adjacency_list(&self) -> AdjacencyList {
        let edges = self
            .inner
            .iter()
            .enumerate()
            .map(|(idx, el)| {
                let pos = Self::position_from_raw(self.width, idx);
                self.adjacent_no_diag(&pos)
                    .into_iter()
                    .filter_map(|o| o.map(|p| Self::position_to_raw(self.width, p)))
                    .filter(|to| el.is_reachable(&self.inner[*to]))
                    .map(|to| {
                        let weight = match el.cmp(&self.inner[to]) {
                            Ordering::Greater => 3,
                            Ordering::Equal => 2,
                            Ordering::Less => 1,
                        };
                        Edge::new(to, weight)
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        AdjacencyList::new(edges)
    }
}

impl TryFrom<&str> for HeightMap {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let width = value.find('\n').ok_or("bad input")?;
        let mut start = Err("no start");
        let mut end = Err("no end");
        let inner = value
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
            .map(|(idx, c)| {
                Elevation::try_from(c).map(|el| match el {
                    Elevation::Start => {
                        start = Ok(idx);
                        el
                    }
                    Elevation::End => {
                        end = Ok(idx);
                        el
                    }
                    _ => el,
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        let height = inner.len() / width;
        let start = start?;
        let end = end?;
        Ok(Self {
            width,
            height,
            inner,
            start,
            end,
        })
    }
}
