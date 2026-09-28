use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub type PathConnections = HashMap<String, u64>;

#[derive(Debug, Default)]
pub struct PathMap {
    nodes: HashMap<String, PathConnections>,
}

impl PathMap {
    fn insert_one_way(&mut self, a: String, b: String, cost: u64) {
        let connections = self
            .nodes
            .entry(a)
            .or_insert(PathConnections::default());

        connections.insert(b, cost);
    }

    pub fn get_shortest_path(&self) -> Option<u64> {
        let mut visited = HashSet::new();
        self.nodes
            .keys()
            .flat_map(move |k| self.get_shortest_path_from(k.as_str(), &mut visited))
            .min()
    }

    fn get_shortest_path_from<'a>(
        &'a self,
        from: &'a str,
        visited: &mut HashSet<&'a str>,
    ) -> Option<u64> {
        if visited.contains(from) {
            return None;
        }

        let path_len = visited.len() + 1;
        if path_len == self.nodes.len() {
            return Some(0);
        } else if !self.nodes.contains_key(from) {
            return None; // Dead end
        }

        visited.insert(from);
        let mut lowest = None;
        for (connection, &conn_cost) in self.nodes[from].iter() {
            if lowest.is_some_and(|l| conn_cost >= l) {
                continue;
            }
            
            let Some(additional_cost) = self.get_shortest_path_from(connection.as_str(), &mut *visited) else {
                continue;
            };
            let total_conn_cost = conn_cost + additional_cost;
            if lowest.is_none_or(|l| total_conn_cost < l)  {
                lowest = Some(total_conn_cost);
            }
        }
        visited.remove(from);

        lowest
    }

    pub fn get_longest_path(&self) -> Option<u64> {
        let mut visited = HashSet::new();
        self.nodes
            .keys()
            .flat_map(move |k| self.get_longest_path_from(k.as_str(), &mut visited))
            .max()
    }

    fn get_longest_path_from<'a>(
        &'a self,
        from: &'a str,
        visited: &mut HashSet<&'a str>,
    ) -> Option<u64> {
        if visited.contains(from) {
            return None;
        }

        let path_len = visited.len() + 1;
        if path_len == self.nodes.len() {
            return Some(0);
        } else if !self.nodes.contains_key(from) {
            return None; // Dead end
        }

        visited.insert(from);
        let mut highest = None;
        for (connection, &conn_cost) in self.nodes[from].iter() {
            let Some(additional_cost) = self.get_longest_path_from(connection.as_str(), &mut *visited) else {
                continue;
            };
            let total_conn_cost = conn_cost + additional_cost;
            if highest.is_none_or(|l| total_conn_cost > l)  {
                highest = Some(total_conn_cost);
            }
        }
        visited.remove(from);

        highest
    }
}

impl FromStr for PathMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .try_fold(PathMap::default(), |mut map, line| -> anyhow::Result<_> {
                let sections: Vec<_> = line
                    .split_whitespace()
                    .collect();

                let a = sections[0].to_string();
                let b = sections[2].to_string();
                let cost = sections[4].parse::<u64>()?;

                map.insert_one_way(a.clone(), b.clone(), cost);
                map.insert_one_way(b, a, cost);
                Ok(map)
            })
    }
}
