use crate::Location;
use crate::LocationOccupancy;
use std::collections::{HashMap, HashSet, VecDeque};
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

    pub fn update_all_joined_locations(
        &mut self,
        location: &Location,
        current: LocationOccupancy,
        updated: LocationOccupancy,
    ) -> (i32, i32) {
        let mut to_visit: VecDeque<Location> = VecDeque::new();
        let mut count_open = 0_i32;
        let mut count_stars = 0_i32;
        to_visit.push_front(location.clone());

        let mut visited = HashSet::new();

        while !to_visit.is_empty() {
            let loc = to_visit.pop_front().unwrap();
            visited.insert(loc.clone());
            to_visit.extend(
                self.location_neighbors(&loc)
                    .iter()
                    .filter(|loc| {
                        (*self.spaces.get(loc).unwrap() == current) && !visited.contains(loc)
                    })
                    .cloned()
                    .collect::<VecDeque<Location>>(),
            );
            self.update_location(loc.clone(), updated);
            count_open += 1;
            count_stars += self
                .location_neighbors(&loc)
                .iter()
                .filter(|loc| matches!(self.spaces.get(loc).unwrap(), LocationOccupancy::STAR))
                .count() as i32;
        }
        (count_open, count_stars)
    }

    pub fn location_neighbors(&self, location: &Location) -> Vec<Location> {
        assert!(self.spaces.contains_key(location));

        let x_ind = self
            .columns
            .iter()
            .position(|&r| r.to_string() == location.x)
            .unwrap();

        let y_ind = self
            .rows
            .iter()
            .position(|&r| r.to_string() == location.y)
            .unwrap();

        let positions = vec![
            (x_ind as i64 - 1, y_ind as i64),
            (x_ind as i64 + 1, y_ind as i64),
            (x_ind as i64, y_ind as i64 - 1),
            (x_ind as i64, y_ind as i64 + 1),
        ];

        let num_cols = self.columns.len() as i64;
        let num_rows = self.rows.len() as i64;

        let mut locations = Vec::<Location>::new();
        // Need to assert this as if I change the board size then
        // the following match function needs to change

        for pos in positions {
            match pos {
                (x, y) if (x >= 0 && x < num_cols) && (y >= 0 && y < num_rows) => {
                    locations.push(Location {
                        x: self.columns[pos.0 as usize].to_string(),
                        y: self.rows[pos.1 as usize].to_string(),
                    })
                }
                _ => {}
            }
        }
        locations
    }

    pub fn get_spaces(&self) -> Vec<Location> {
        let mut locations = self.spaces.keys().cloned().collect::<Vec<Location>>();

        locations.sort();

        locations
    }

    pub fn space(&self, loc: &Location) -> Option<&LocationOccupancy> {
        self.spaces.get(loc)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = format!("{:-^1$}", "  MAP OF THE GALAXY  ", 37);
        grid.push('\n');
        grid.push_str(format!("{:*^1$}", "", 37).as_str());
        grid.push('\n');
        grid.push(' ');
        let space = "  ";
        grid.push_str(space);
        for c in &self.columns {
            grid.push(*c);
            grid.push_str(space);
        }
        grid.push('\n');

        for r in &self.rows {
            let mut row = r.to_string();

            for c in &self.columns {
                row.push_str(space);
                let loc = Location {
                    x: c.clone().to_string(),
                    y: r.clone().to_string(),
                };
                row.push_str(format!("{}", self.spaces.get(&loc).unwrap()).as_str());
            }
            row.push('\n');
            grid.push_str(row.as_str());
        }
        write!(f, "{}", grid)
    }
}

#[test]
fn test_board_creation() {
    let _board = Board::new();
}

#[test]
fn test_get_neighbors_upper_left_corner() {
    let board = Board::new();
    let loc = Location {
        x: board.columns[0].to_string(),
        y: board.rows[0].to_string(),
    };

    let answer = vec![
        Location {
            x: "B".to_string(),
            y: "1".to_string(),
        },
        Location {
            x: "A".to_string(),
            y: "2".to_string(),
        },
    ];
    let result = board.location_neighbors(&loc);

    assert_eq!(result, answer)
}

#[test]
fn test_get_neighbors_upper_right_corner() {
    let board = Board::new();
    let loc = Location {
        x: board.columns[board.columns.len() - 1].to_string(),
        y: board.rows[0].to_string(),
    };

    let answer = vec![
        Location {
            x: "K".to_string(),
            y: "1".to_string(),
        },
        Location {
            x: "L".to_string(),
            y: "2".to_string(),
        },
    ];
    let result = board.location_neighbors(&loc);

    assert_eq!(result, answer)
}

#[test]
fn test_get_neighbors_lower_left_corner() {
    let board = Board::new();
    let loc = Location {
        x: board.columns[0].to_string(),
        y: board.rows[board.rows.len() - 1].to_string(),
    };

    let answer = vec![
        Location {
            x: board.columns[1].to_string(),
            y: board.rows[board.rows.len() - 1].to_string(),
        },
        Location {
            x: board.columns[0].to_string(),
            y: board.rows[board.rows.len() - 2].to_string(),
        },
    ];
    let result = board.location_neighbors(&loc);

    assert_eq!(result, answer)
}

#[test]
fn test_get_neighbors_lower_right_corner() {
    let board = Board::new();
    let loc = Location {
        x: board.columns[board.columns.len() - 1].to_string(),
        y: board.rows[board.rows.len() - 1].to_string(),
    };

    let answer = vec![
        Location {
            x: board.columns[board.columns.len() - 2].to_string(),
            y: board.rows[board.rows.len() - 1].to_string(),
        },
        Location {
            x: board.columns[board.columns.len() - 1].to_string(),
            y: board.rows[board.rows.len() - 2].to_string(),
        },
    ];
    let result = board.location_neighbors(&loc);

    assert_eq!(result, answer)
}
