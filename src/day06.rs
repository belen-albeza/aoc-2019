use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use indextree::{Arena, NodeId};

type Orbit = (String, String);

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<Orbit> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut chunks = line.split(")");
            (
                chunks.next().unwrap().to_string(),
                chunks.next().unwrap().to_string(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct OrbitalMap {
    tree: Arena<String>,
    root: NodeId,
}

impl OrbitalMap {
    pub fn new() -> Self {
        let mut tree = Arena::new();
        let root = tree.new_node(String::from("COM"));
        Self { tree, root }
    }

    fn add_orbit(&mut self, center: String, orbiting: String) {
        let center_node = self
            .id_for_value(&center)
            .unwrap_or_else(|| self.tree.new_node(center));

        let orbiting_node = self
            .id_for_value(&orbiting)
            .unwrap_or_else(|| self.tree.new_node(orbiting));

        center_node.append(orbiting_node, &mut self.tree);
    }

    fn id_for_value(&self, value: &str) -> Option<NodeId> {
        self.tree
            .iter()
            .find(|x| !x.is_removed() && *x.get() == value)
            .map(|x| self.tree.get_node_id(x).unwrap())
    }

    fn value_for_id(&self, id: NodeId) -> String {
        self.tree.get(id).unwrap().get().to_owned()
    }

    fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        self.find_subpath(from, to, &vec![])
    }

    fn find_subpath(&self, from: &str, to: &str, visited: &[String]) -> Option<Vec<String>> {
        let current = self.id_for_value(from).unwrap();
        let target = self.id_for_value(to).unwrap();

        let path: Vec<String> = vec![self.value_for_id(current)];
        if current == target {
            return Some(path);
        }

        let upward = current.ancestors(&self.tree).skip(1).next();
        let downward = current.children(&self.tree);

        let mut potential: Vec<NodeId> = match upward {
            Some(node_id) => vec![node_id],
            None => vec![],
        };
        potential.extend(downward);

        let found: Vec<Vec<String>> = potential
            .iter()
            .map(|node_id| {
                if !visited.contains(&self.value_for_id(*node_id)) {
                    let new_visited = vec![visited, &path].concat();
                    self.find_subpath(&self.value_for_id(*node_id), to, &new_visited)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        found.get(0).map(|subpath| [path, subpath.clone()].concat())
    }
}

impl From<&[Orbit]> for OrbitalMap {
    fn from(orbits: &[Orbit]) -> Self {
        let mut orbital_map = Self::new();
        for (center, orbiting) in orbits {
            orbital_map.add_orbit(center.to_owned(), orbiting.to_owned());
        }

        orbital_map
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Orbit]) -> u64 {
    let orbital_map = OrbitalMap::from(input);

    orbital_map
        .root
        .descendants(&orbital_map.tree)
        .skip(1)
        .map(|x| (x.ancestors(&orbital_map.tree).count() as i64 - 1) as u64)
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Orbit]) -> Option<u64> {
    let orbital_map = OrbitalMap::from(input);
    let path = orbital_map.find_path("YOU", "SAN");

    path.map(|x| (x.len() - 3) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_WITH_SANTA: &'static str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    fn to_string_vec(list: Vec<&str>) -> Vec<String> {
        list.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_parse_input() {
        let input = "COM)B\nB)C\nC)D";
        assert_eq!(
            parse_input(input),
            vec![("COM", "B"), ("B", "C"), ("C", "D")]
                .iter()
                .map(|(x, y)| (x.to_string(), y.to_string()))
                .collect::<Vec<Orbit>>(),
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = parse_input(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
        );
        assert_eq!(solve_part1(&input), 42);
    }

    #[test]
    fn test_find_path() {
        let input = parse_input(INPUT_WITH_SANTA);
        let orbital_map = OrbitalMap::from(&input[..]);

        assert_eq!(
            orbital_map.find_path("B", "B"),
            Some(to_string_vec(vec!["B"]))
        );
        assert_eq!(
            orbital_map.find_path("B", "G"),
            Some(to_string_vec(vec!["B", "G"]))
        );
        assert_eq!(
            orbital_map.find_path("G", "B"),
            Some(to_string_vec(vec!["G", "B"]))
        );
        assert_eq!(
            orbital_map.find_path("B", "H"),
            Some(to_string_vec(vec!["B", "G", "H"]))
        );
        assert_eq!(
            orbital_map.find_path("H", "B"),
            Some(to_string_vec(vec!["H", "G", "B"]))
        );
        assert_eq!(
            orbital_map.find_path("YOU", "SAN"),
            Some(to_string_vec(vec!["YOU", "K", "J", "E", "D", "I", "SAN"]))
        );
    }

    #[test]
    fn test_solve_part2() {
        let input = parse_input(INPUT_WITH_SANTA);
        assert_eq!(solve_part2(&input), Some(4));
    }
}
