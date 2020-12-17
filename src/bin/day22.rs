use std::collections::{HashMap, VecDeque};

use snafu::{ResultExt, Snafu};

use lazy_static::lazy_static;
use regex::Regex;

use aoc2016::map::Map;

lazy_static! {
    static ref RE_NODE: Regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Int format error for '{}': {}", data, source))]
    ParseInt {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Invalid node: '{}'", data))]
    ParseNode { data: String },
}

#[derive(Debug, Clone)]
struct Node {
    x: i8,
    y: i8,
    size: u16,
    used: u16,
    avail: u16,
    use_pct: u16,
}

impl std::str::FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RE_NODE.captures(s.trim()).ok_or(Error::ParseNode {
            data: s.to_string(),
        })?;

        let x = captures.get(1).unwrap().as_str();
        let y = captures.get(2).unwrap().as_str();
        let size = captures.get(3).unwrap().as_str();
        let used = captures.get(4).unwrap().as_str();
        let avail = captures.get(5).unwrap().as_str();
        let use_pct = captures.get(6).unwrap().as_str();

        let x: i8 = x.parse().context(ParseInt {
            data: x.to_string(),
        })?;
        let y: i8 = y.parse().context(ParseInt {
            data: y.to_string(),
        })?;
        let size: u16 = size.parse().context(ParseInt {
            data: size.to_string(),
        })?;
        let used: u16 = used.parse().context(ParseInt {
            data: used.to_string(),
        })?;
        let avail: u16 = avail.parse().context(ParseInt {
            data: avail.to_string(),
        })?;
        let use_pct: u16 = use_pct.parse().context(ParseInt {
            data: use_pct.to_string(),
        })?;

        Ok(Node {
            x,
            y,
            size,
            used,
            avail,
            use_pct,
        })
    }
}

impl Node {
    fn can_send_to(&self, target: &Node) -> bool {
        self.used > 0 && target.avail >= self.used
    }
}

#[derive(Debug, Clone)]
struct NodeStatus {
    used: u16,
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {:3}", self.used)
    }
}

#[derive(Debug, Clone)]
struct State {
    map: Map<[i8; 2], NodeStatus>,
    target_pos: [i8; 2],
}

impl State {
    fn get_neighbors(&self, node_defs: &HashMap<[i8; 2], Node>) -> Vec<State> {
        let mut out = Vec::new();

        let (min, max) = self.map.get_extent();
        for i in min[0]..=max[0] {
            for j in min[1]..=max[1] {
                let pos_source = [i, j];
                if let Some(stat_source) = self.map.get(&pos_source) {
                    if stat_source.used <= 0 {
                        continue;
                    }

                    for (iofs, jofs) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let pos_target = [i + iofs, j + jofs];

                        if let Some(stat_target) = self.map.get(&pos_target) {
                            let nd_target = &node_defs[&pos_target];

                            if stat_target.used + stat_source.used <= nd_target.size {
                                // move data from pos_source to pos_target

                                let mut new_state = self.clone();
                                new_state.map.set(pos_source, NodeStatus { used: 0 });
                                new_state.map.set(
                                    pos_target,
                                    NodeStatus {
                                        used: stat_target.used + stat_source.used,
                                    },
                                );

                                // if moving the desired data, also update pointer
                                if pos_source == self.target_pos {
                                    new_state.target_pos = pos_target;
                                }

                                out.push(new_state);
                            }
                        }
                    }
                }
            }
        }

        out
    }
}

fn main() -> Result<()> {
    let nodes: Vec<Node> = std::fs::read_to_string("data/day22/input")
        .context(Io)?
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut n_viable = 0;
    for (i, n) in nodes.iter().enumerate() {
        for (j, m) in nodes.iter().enumerate() {
            if i != j && n.can_send_to(m) {
                n_viable += 1;
            }
        }
    }

    println!("Part 1: got {} viable pairs", n_viable);

    // convert to useful representation
    let mut map: Map<[i8; 2], NodeStatus> = Map::new();
    let mut node_defs: HashMap<[i8; 2], Node> = HashMap::new();
    for n in nodes {
        map.set([n.y, n.x], NodeStatus { used: n.used });
        node_defs.insert([n.y, n.x], n);
    }

    let (_, max) = map.get_extent();
    let target_pos = [0, max[1]];

    let initial_state = State { map, target_pos };

    let mut queue = VecDeque::new();
    queue.push_back((0, initial_state));

    let mut max_steps = 0;
    while let Some((steps, current)) = queue.pop_front() {
        if steps > max_steps {
            max_steps = steps;
            println!("{}", steps);
        }

        if current.target_pos == [0, 0] {
            println!("{}\nPart 2: found solution in {} steps", current.map, steps);
        }

        for n in current.get_neighbors(&node_defs) {
            queue.push_back((steps + 1, n));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        Ok(())
    }
}
