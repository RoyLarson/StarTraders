

use std::collections::HashMap;
use rand::prelude::*;
use rand_pcg::Pcg64;

use std::fmt;
use crate::location::Location;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Company{
    ALTAIR,
    BETELGEUSE,
    CAPELLA,
    DENEBOLA,
    ERIDANI,
}

pub enum LocationOccupancy{
    OPEN,
    STAR,
    COMPANY(Company)
}

#[derive(Debug,Clone)]
pub struct Moves(Vec<Location>);

impl fmt::Display for Moves{
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
        for move_loc in &self.0[0..self.0.len()]{
            if move_loc == loc { return true}
        }
        false
    }
} 
pub struct Board {
    pub spaces: HashMap<Location,LocationOccupancy>,
    columns: Vec<char>,
    rows: Vec<char>
}

impl Board {
    pub fn new(seed: usize)->Board {
        let mut rng= Pcg64::seed_from_u64(seed as u64);
        let columns: Vec<char> = "ABCDEFGHIJKL".chars().collect();
        let rows: Vec<char> = "123456789".chars().collect();
        let mut spaces: HashMap<Location,LocationOccupancy> = HashMap::new();

        for l in &columns {
            for n in &rows{
                let rand_num:f64 = rng.gen();
                if rand_num<0.05{
                    spaces.insert(Location{x:l.to_string(), y:l.to_string()}, LocationOccupancy::STAR);
                } else {
                    spaces.insert(Location{x:l.to_string(), y:n.to_string()}, LocationOccupancy::OPEN);
                }
            }
        }

        Board{spaces, columns, rows}

    }
    pub fn get_legal_moves(&self)->Moves {
        let mut open_locations= Vec::<Location>::new();
        for (location, occupation) in &self.spaces{
            match occupation{
                LocationOccupancy::OPEN=>{
                    open_locations.push(location.clone());
                }
                _=>{}
            }
        }
        let mut moves = Vec::<Location>::new();
        for loc in open_locations.choose_multiple(&mut rand::thread_rng(), 6){
            moves.push(loc.clone())
        }
        Moves(moves)
    }
    fn location_neighbors(&self, location:&Location)->Vec<Location>{
        let mut locations = Vec::<Location>::new();
        let x_ind = self.columns.iter().position(|&r| r.to_string() == location.x).unwrap();
        let y_ind = self.columns.iter().position(|&r| r.to_string() == location.y).unwrap();
        let positions = vec![(x_ind-1, y_ind), (x_ind+1, y_ind), (x_ind, y_ind-1), (x_ind, y_ind+1)];
        for pos in positions{
            match pos{
                (0..=12, 0..=9)=>{
                    locations.push(Location{x:self.columns[pos.0].to_string(), y:self.rows[pos.1].to_string()})
                }
                _=>{}
            }
        }
        locations
    }
}

#[test]
fn test_board_creation(){
    let _board = Board::new(42);
}