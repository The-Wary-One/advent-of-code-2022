use crate::heap::MinHeap;

#[derive(Debug)]
pub struct Edge {
    to: usize,
    weight: usize,
}

impl Edge {
    pub fn new(to: usize, weight: usize) -> Self {
        Self { to, weight }
    }
}

pub struct AdjacencyList {
    edges: Box<[Box<[Edge]>]>,
}

impl AdjacencyList {
    pub fn new(edges: Box<[Box<[Edge]>]>) -> Self {
        Self { edges }
    }

    pub fn dijkstra_with_priority_queue(&self, start: usize, end: usize) -> Option<Box<[usize]>> {
        let mut pq = MinHeap::new();
        let mut path_and_cost = vec![None; self.edges.len()].into_boxed_slice();

        path_and_cost[start] = Some((start, 0));
        for edge in self.edges[start].iter() {
            pq.push(PQItem {
                from: start,
                to: edge.to,
                cost: edge.weight,
            });
        }

        while let Some(item) = pq.pop() {
            let parent = item.from;
            let cur = item.to;
            let cur_cost = item.cost;

            let cur_pc = &mut path_and_cost[cur];
            // Node already visited.
            if cur_pc.is_some() {
                continue;
            }

            *cur_pc = Some((parent, cur_cost));
            // End reached.
            if cur == end {
                break;
            }

            for edge in self.edges[cur].iter() {
                pq.push(PQItem {
                    from: cur,
                    to: edge.to,
                    cost: cur_cost + edge.weight,
                });
            }
        }

        let (mut origin, _) = &path_and_cost[end]?;
        let mut v = vec![end];
        loop {
            (origin, _) = path_and_cost[origin].expect("safe");
            v.push(origin);
            if origin == start {
                break;
            }
        }
        v.reverse();
        Some(v.into_boxed_slice())
    }
}

#[derive(Ord, Eq, Debug)]
struct PQItem {
    from: usize,
    to: usize,
    cost: usize,
}

impl PartialOrd for PQItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for PQItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
