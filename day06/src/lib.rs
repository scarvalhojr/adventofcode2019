use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

type Object = String;

const YOU: &str = "YOU";
const SANTA: &str = "SAN";
const CENTER_OF_MASS: &str = "COM";

pub struct Orbit {
    orbited: Object,
    orbiting: Object,
}

pub fn part1(orbits: &[Orbit]) -> usize {
    let mut queue = orbits.iter().collect::<VecDeque<_>>();
    let mut orbit_count = HashMap::new();
    orbit_count.insert(CENTER_OF_MASS.to_string(), 0);
    while let Some(orbit) = queue.pop_front() {
        if let Some(&count) = orbit_count.get(&orbit.orbited) {
            orbit_count.insert(orbit.orbiting.to_string(), count + 1);
        } else {
            queue.push_back(orbit);
        }
    }
    orbit_count.values().sum()
}

type TransferMap = HashMap<Object, HashSet<Object>>;

fn build_transfer_map(orbits: &[Orbit]) -> TransferMap {
    let mut transfers = TransferMap::new();
    for orb in orbits {
        transfers
            .entry(orb.orbited.to_string())
            .and_modify(|t| {
                t.insert(orb.orbiting.to_string());
            })
            .or_insert_with(|| {
                vec![orb.orbiting.to_string()].into_iter().collect()
            });
        transfers
            .entry(orb.orbiting.to_string())
            .and_modify(|t| {
                t.insert(orb.orbited.to_string());
            })
            .or_insert_with(|| {
                vec![orb.orbited.to_string()].into_iter().collect()
            });
    }
    transfers
}

pub fn part2(orbits: &[Orbit]) -> Option<usize> {
    let start = orbits
        .iter()
        .find(|orbit| orbit.orbiting == YOU)
        .map(|orbit| &orbit.orbited)?;
    let target = orbits
        .iter()
        .find(|orbit| orbit.orbiting == SANTA)
        .map(|orbit| &orbit.orbited)?;
    let transfers = build_transfer_map(orbits);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), 0));
    while let Some((object, distance)) = queue.pop_front() {
        if &object == target {
            return Some(distance);
        }
        visited.insert(object.to_string());
        for next_object in transfers.get(&object)?.iter() {
            if !visited.contains(next_object) {
                queue.push_back((next_object.to_string(), distance + 1));
            }
        }
    }
    None
}

impl FromStr for Orbit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut objects = s.split(')').map(|obj| obj.to_string());
        let orbited =
            objects.next().ok_or_else(|| "Invalid orbit".to_string())?;
        let orbiting =
            objects.next().ok_or_else(|| "Invalid orbit".to_string())?;
        Ok(Self { orbited, orbiting })
    }
}
