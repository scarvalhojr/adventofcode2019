use std::str::FromStr;
use std::collections::HashSet;

const GRID_SIZE: i8 = 5;
const GRID_CENTER: i8 = 2;

type Position = (i8, i8);
type Position3D = (i8, i8, i8);

#[derive(Clone)]
pub struct Area {
    bugs: HashSet<Position>,
}

impl Area {
    fn infested_neighbours(&self, position: Position) -> usize {
        let (pos_x, pos_y) = position;
        [
            (pos_x - 1, pos_y),
            (pos_x + 1, pos_y),
            (pos_x, pos_y - 1),
            (pos_x, pos_y + 1),
        ].iter()
            .filter(|position| self.bugs.contains(position))
            .count()
    }

    fn bug_lives(&self, position: Position) -> bool {
        self.bugs.contains(&position) && self.infested_neighbours(position) == 1
    }

    fn becomes_infested(&self, position: Position) -> bool {
        let inf_count = self.infested_neighbours(position);
        !self.bugs.contains(&position) && (inf_count == 1 || inf_count == 2)
    }

    fn next_area(&self) -> Self {
        let bugs = (0..GRID_SIZE)
            .flat_map(|pos_x| (0..GRID_SIZE).map(move |pos_y| (pos_x, pos_y)))
            .filter(|&pos| self.bug_lives(pos) || self.becomes_infested(pos))
            .collect();
        Self { bugs }
    }

    fn bio_rating(&self) -> u32 {
        self.bugs
            .iter()
            .map(|(pos_x, pos_y)| {
                2_u32.pow((pos_x + pos_y * GRID_SIZE) as u32)
            })
            .sum()
    }

    fn get_area_3d(&self) -> Area3D {
    	let bugs = self.bugs
    	    .iter()
    	    .map(|(pos_x, pos_y)| (*pos_x, *pos_y, 0))
    	    .collect();
    	Area3D { bugs }
    }
}

pub fn part1(area: &Area) -> u32 {
    let mut ratings = HashSet::new();
    let mut area = area.clone();
    while ratings.insert(area.bio_rating()) {
        area = area.next_area();
    }
    area.bio_rating()
}

struct Area3D {
	bugs: HashSet<Position3D>,
}

impl Area3D {
	fn bug_count(&self) -> usize {
	    self.bugs.len()
	}

	fn next_area(&self) -> Self {
	    let min_level = self.bugs
	        .iter()
	        .map(|(_, _, level)| *level)
	        .min()
	        .unwrap_or(0)
	        - 1;
	    let max_level = self.bugs
	        .iter()
	        .map(|(_, _, level)| *level)
	        .max()
	        .unwrap_or(0)
	        + 1;
	    let bugs = (min_level..=max_level)
	        .flat_map(|level| {
	            (0..GRID_SIZE)
	                .flat_map(move |pos_x| {
	                    (0..GRID_SIZE)
	                        .map(move |pos_y| (pos_x, pos_y, level))
	                })
	        })
	        .filter(|&(pos_x, pos_y, _)| pos_x != 2 || pos_y != 2)
	        .filter(|&pos| self.bug_lives(pos) || self.becomes_infested(pos))
	        .collect();
	    Self { bugs }
	}

	fn infested_neighbours(&self, position: Position3D) -> usize {
	    let (pos_x, pos_y, level) = position;
	    let mut neighbours = vec![
	        (pos_x - 1, pos_y, level),
	        (pos_x + 1, pos_y, level),
	        (pos_x, pos_y - 1, level),
	        (pos_x, pos_y + 1, level),
	    ];

	    match pos_x {
	    	0 => {
	    		neighbours.push((GRID_CENTER - 1, GRID_CENTER, level - 1));
	    	}
	    	1 => {
	    		if pos_y == GRID_CENTER {
	    		    for y in 0..GRID_SIZE {
	    			    neighbours.push((0, y, level + 1));
	    			}
	    		}
	    	}
	    	GRID_CENTER => {
	    		if pos_y == GRID_CENTER - 1 {
	    			for x in 0..GRID_SIZE {
	    		        neighbours.push((x, 0, level + 1));
	    			}
	    		} else if pos_y == GRID_CENTER + 1 {
	    			for x in 0..GRID_SIZE {
	    			    neighbours.push((x, GRID_SIZE - 1, level + 1));
	    		    }
	    		}
	    	}
            3 => {
	    		if pos_y == GRID_CENTER {
	    		    for y in 0..GRID_SIZE {
	    			    neighbours.push((GRID_SIZE - 1, y, level + 1));
	    			}
	    		}
	    	}
	    	4 => {
	    		neighbours.push((GRID_CENTER + 1, GRID_CENTER, level - 1));
	    	}
	    	_ => {}
	    }

        if pos_y == 0 {
        	neighbours.push((2, 1, level - 1));
        } else if pos_y == GRID_SIZE - 1 {
        	neighbours.push((2, 3, level - 1));
        }
        
	    neighbours
	        .iter()
	        .filter(|position| self.bugs.contains(position))
	        .count()
	}

	fn bug_lives(&self, position: Position3D) -> bool {
	    self.bugs.contains(&position) && self.infested_neighbours(position) == 1
	}
	
	fn becomes_infested(&self, position: Position3D) -> bool {
	    let inf_count = self.infested_neighbours(position);
	    !self.bugs.contains(&position) && (inf_count == 1 || inf_count == 2)
	}
}

pub fn part2(area: &Area) -> usize {
    let mut area = area.get_area_3d();
    for _ in 0..200 {
    	area = area.next_area();
    }
    area.bug_count()
}

impl FromStr for Area {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bugs = s.lines()
            .take(GRID_SIZE as usize)
            .enumerate()
            .flat_map(|(pos_y, line)| {
                line.chars().take(GRID_SIZE as usize).enumerate().map(move |(pos_x, ch)| {
                    ((pos_x as i8, pos_y as i8), ch)
                })
            })
            .filter(|(_, ch)| *ch == '#')
            .map(|(position, _)| position)
            .collect();
        Ok(Self { bugs })
    }
}
