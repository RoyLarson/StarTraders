use crate::location::Location;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Company {
    ALTAIR,
    BETELGEUSE,
    CAPELLA,
    DENEBOLA,
    ERIDANI,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LocationOccupancy {
    OPEN,
    PLAYED,
    STAR,
    COMPANY(Company),
}

#[derive(Debug, Clone)]
pub struct Moves(pub Vec<Location>);

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut comma_separated = String::new();

        for loc in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(format!("{}", &loc).as_str());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(format!("{}", &self.0[self.0.len() - 1]).as_str());
        write!(f, "{}", comma_separated)
    }
}

impl Moves {
    pub fn contains(&self, loc: &Location) -> bool {
        for move_loc in &self.0[0..self.0.len()] {
            if move_loc == loc {
                return true;
            }
        }
        false
    }
}
pub struct Board {
    pub spaces: HashMap<Location, LocationOccupancy>,
    columns: Vec<char>,
    rows: Vec<char>,
}

impl Board {
    pub fn new() -> Board {
        let columns: Vec<char> = "ABCDEFGHIJKL".chars().collect();
        let rows: Vec<char> = "123456789".chars().collect();
        let mut spaces: HashMap<Location, LocationOccupancy> = HashMap::new();

        for l in &columns {
            for n in &rows {
                spaces.insert(
                    Location {
                        x: l.to_string(),
                        y: n.to_string(),
                    },
                    LocationOccupancy::OPEN,
                );
            }
        }

        Board {
            spaces,
            columns,
            rows,
        }
    }

    pub fn update_location(&mut self, location: Location, occupancy: LocationOccupancy) {
        self.spaces.entry(location).or_insert(occupancy);
    }

    pub fn location_neighbors(&self, location: &Location) -> Vec<Location> {
        let mut locations = Vec::<Location>::new();
        let x_ind = self
            .columns
            .iter()
            .position(|&r| r.to_string() == location.x)
            .unwrap();
        let y_ind = self
            .columns
            .iter()
            .position(|&r| r.to_string() == location.y)
            .unwrap();
        let positions = vec![
            (x_ind - 1, y_ind),
            (x_ind + 1, y_ind),
            (x_ind, y_ind - 1),
            (x_ind, y_ind + 1),
        ];
        for pos in positions {
            match pos {
                (0..=12, 0..=9) => locations.push(Location {
                    x: self.columns[pos.0].to_string(),
                    y: self.rows[pos.1].to_string(),
                }),
                _ => {}
            }
        }
        locations
    }
    pub fn get_spaces(&self) -> Vec<Location> {
        let locations = self.spaces.keys().cloned().collect::<Vec<Location>>();
        locations
    }
    pub fn space(&self, loc: Location) -> Option<LocationOccupancy> {
        match self.spaces.get(&loc) {
            Some(occ) => return Some(*occ),
            None => return None,
        }
    }
}

#[test]
fn test_board_creation() {
    let _board = Board::new();
}
