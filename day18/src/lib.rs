use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::str::FromStr;
use Direction::*;

type Key = char;
type Distance = u16;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    pos_x: i8,
    pos_y: i8,
}

impl Position {
    fn new(pos_x: i8, pos_y: i8) -> Self {
        Self { pos_x, pos_y }
    }

    fn go(self, direction: Direction) -> Self {
        match direction {
            Up => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y - 1,
            },
            Down => Self {
                pos_x: self.pos_x,
                pos_y: self.pos_y + 1,
            },
            Left => Self {
                pos_x: self.pos_x - 1,
                pos_y: self.pos_y,
            },
            Right => Self {
                pos_x: self.pos_x + 1,
                pos_y: self.pos_y,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Area {
    Entrance,
    Open,
    Wall,
    Door(Key),
    Key(Key),
}

impl Area {
    fn is_key(self) -> bool {
        match self {
            Area::Key(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Area {
    type Error = &'static str;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            '@' => Ok(Area::Entrance),
            '.' => Ok(Area::Open),
            '#' => Ok(Area::Wall),
            ch => {
                if ch.is_ascii_uppercase() {
                    Ok(Area::Door(ch.to_ascii_lowercase()))
                } else if ch.is_ascii_lowercase() {
                    Ok(Area::Key(ch))
                } else {
                    Err("Invalid area")
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct VaultMap {
    area: HashMap<Position, Area>,
}

#[derive(Clone, Default)]
struct KeyConnection {
    key: Key,
    distance: Distance,
    required_keys: BTreeSet<Key>,
}

impl KeyConnection {
    fn clone_with_key(&self, key: Key) -> Self {
        Self {
            key,
            distance: self.distance,
            required_keys: self.required_keys.clone(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    positions: Vec<Position>,
    distance: Distance,
    keys: BTreeSet<Key>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since this is used in a max-heap, the order is based first
        // on the reverse of the distance then number of collected keys;
        // this order does not fully agree with derived Eq
        self.distance
            .cmp(&other.distance)
            .reverse()
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, Eq, PartialEq)]
struct StateSignature {
    positions: Vec<Position>,
    keys: BTreeSet<Key>,
}

impl State {
    fn new(positions: Vec<Position>) -> Self {
        Self {
            positions,
            distance: 0,
            keys: BTreeSet::new(),
        }
    }

    fn extend(
        &self,
        index: usize,
        position: Position,
        key: Key,
        distance: Distance,
    ) -> Self {
        let mut positions = self.positions.clone();
        positions[index] = position;
        let mut keys = self.keys.clone();
        keys.insert(key);
        Self {
            positions,
            distance: self.distance + distance,
            keys,
        }
    }

    fn signature(&self) -> StateSignature {
        StateSignature {
            positions: self.positions.clone(),
            keys: self.keys.clone(),
        }
    }
}

impl VaultMap {
    pub fn shortest_path_to_all_keys(&self) -> Option<Distance> {
        let key_count = self.count_keys();
        let connections = self.all_key_connections();
        let mut state_dist = HashMap::new();
        let mut min_dist = None;

        let entrances = self
            .area
            .iter()
            .filter(|&(_, area)| *area == Area::Entrance)
            .map(|(position, _)| *position)
            .collect();
        let mut queue = BinaryHeap::new();
        queue.push(State::new(entrances));

        while let Some(state) = queue.pop() {
            if state.keys.len() == key_count {
                if min_dist.map(|min| min > state.distance).unwrap_or(true) {
                    // New best path found
                    min_dist = Some(state.distance);
                }
                continue;
            }

            if min_dist.map(|min| min < state.distance).unwrap_or(false) {
                // Current path is worse than one already found
                continue;
            }

            let state_sig = state.signature();
            if let Some(known_distance) = state_dist.get_mut(&state_sig) {
                if *known_distance <= state.distance {
                    // Equivalent state already reached with shorter path
                    continue;
                } else {
                    *known_distance = state.distance;
                }
            } else {
                state_dist.insert(state_sig, state.distance);
            }

            for (index, position) in state.positions.iter().enumerate() {
                for (next_position, conn) in connections.get(position).unwrap()
                {
                    if state.keys.contains(&conn.key) {
                        // Key already collected
                        continue;
                    }
                    if conn.required_keys.is_subset(&state.keys) {
                        queue.push(state.extend(
                            index,
                            *next_position,
                            conn.key,
                            conn.distance,
                        ));
                    }
                }
            }
        }

        min_dist
    }

    fn all_key_connections(
        &self,
    ) -> HashMap<Position, HashMap<Position, KeyConnection>> {
        self.area
            .iter()
            .filter(|(_, &area)| match area {
                Area::Entrance => true,
                Area::Key(_) => true,
                _ => false,
            })
            .map(|(&position, _)| {
                (position, self.key_connections_from(position))
            })
            .collect()
    }

    fn key_connections_from(
        &self,
        start: Position,
    ) -> HashMap<Position, KeyConnection> {
        let mut connections = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, KeyConnection::default()));
        while let Some((position, mut connection)) = queue.pop_front() {
            connection.distance += 1;
            for next_position in self.neighbours(position) {
                if !visited.insert(next_position) {
                    // Position already visited
                    continue;
                }
                let mut next_connection = connection.clone();
                match self.area.get(&next_position) {
                    Some(Area::Key(key)) => {
                        // Found a new connection to a key
                        connections.insert(
                            next_position,
                            next_connection.clone_with_key(*key),
                        );
                    }
                    Some(Area::Door(key)) => {
                        next_connection.required_keys.insert(*key);
                    }
                    Some(Area::Open) => {}
                    Some(Area::Entrance) => {}
                    _ => {
                        // Wall or invalid position
                        continue;
                    }
                };
                queue.push_back((next_position, next_connection));
            }
        }

        connections
    }

    fn neighbours(&self, position: Position) -> Vec<Position> {
        [Up, Down, Left, Right]
            .iter()
            .map(|direction| position.go(*direction))
            .filter(|&pos| self.area.contains_key(&pos))
            .collect()
    }

    fn count_keys(&self) -> usize {
        self.area.iter().filter(|(_, area)| area.is_key()).count()
    }
}

impl FromStr for VaultMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let area = s
            .lines()
            .enumerate()
            .flat_map(|(pos_y, line)| {
                line.chars().enumerate().map(move |(pos_x, ch)| {
                    // TODO: handle integer conversion errors
                    let position = Position::new(pos_x as i8, pos_y as i8);
                    Area::try_from(ch).map(|area| (position, area))
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { area })
    }
}
