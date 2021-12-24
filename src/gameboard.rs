use crate::Location;
use crate::LocationOccupancy;
use std::collections::HashMap;
use std::fmt;

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
        self.spaces.insert(location, occupancy);
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let space = "   ";
        let mut grid = String::from("    ");
        for c in &self.columns {
            grid.push(c.clone());
            grid.push_str(space.clone());
        }
        grid.push_str("\n");

        for r in &self.rows {
            let mut row = String::from(r.to_string());

            for c in &self.columns {
                row.push_str(space);
                let loc = Location {
                    x: c.clone().to_string(),
                    y: r.clone().to_string(),
                };
                row.push_str(format!("{}", self.spaces.get(&loc).unwrap()).as_str());
            }
            row.push_str("\n");
            grid.push_str(row.as_str());
        }
        write!(f, "{}", grid)
    }
}

#[test]
fn test_board_creation() {
    let _board = Board::new();
}
