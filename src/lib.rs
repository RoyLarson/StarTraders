use std::collections::HashMap;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::str::FromStr;



#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Company{
    ALTAIR,
    BETELGEUSE,
    CAPELLA,
    DENEBOLA,
    ERIDANI,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
    balance: usize,
    stocks: HashMap<Company,usize>,    
}

impl Player{
    pub fn new(name:String)->Player{
        let mut stocks = HashMap::new();
        stocks.insert(Company::ALTAIR,0);
        stocks.insert(Company::BETELGEUSE,0);
        stocks.insert(Company::CAPELLA,0);
        stocks.insert(Company::DENEBOLA,0);
        stocks.insert(Company::ERIDANI,0);
        Player{name, balance:100, stocks}

    }
}

pub enum LocationOccupancy{
    OPEN,
    STAR,
    COMPANY(Company)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Location{
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone)]
pub enum ParsePointError {
    FailedParse(String),
    Not2Dimensional(usize),
    NonNumeric,
}

impl FromStr for Location {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s
            .trim_matches(|p| p == '(' || p == ')')
            .trim()
            .replace(|p| p == ' ', "");
        {
            if !clean_s.contains(
                |c| {
                        ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&c)
                    }
                ) 
                {
                return Err(ParsePointError::NonNumeric);
                }
        }
        let coords:String = clean_s.chars().collect::<String>();

        if coords.len() != 2 {
            return Err(ParsePointError::Not2Dimensional(coords.len()));
        }
        let x = Some(match &coords.chars().nth(0) {
            Some(c) => c.to_string(),
            None => return Err(ParsePointError::FailedParse(format!("X value is not a char "))),
            }
        );

        let y = Some(match &coords.chars().nth(1) {
            Some(c) => c.to_string(),
            None => return Err(ParsePointError::FailedParse(format!("Y value is not a char"))),
            }
         );
        
        if y.is_none() || x.is_none() {
            return Err(ParsePointError::FailedParse(format!("Not enough coordinates to be valid: {:?}", &coords)));
        }
        Ok(Location {x:x.unwrap(), y:y.unwrap()})
        
    }
}

#[test]
fn test_location_from_str(){
    let loc = Location::from_str("A1").unwrap();
    assert_eq!(&loc.x, "A");
    assert_eq!(&loc.y, "1");

}


pub struct Board {
    pub spaces: HashMap<Location,LocationOccupancy>
}

impl Board {
    pub fn new(seed: usize)->Board {
        let mut rng= Pcg64::seed_from_u64(seed as u64);
        let columns = "ABCDEFGHIJKL".to_string();
        let rows = "123456789".to_string();
        let mut spaces: HashMap<Location,LocationOccupancy> = HashMap::new();

        for l in columns.chars() {
            for n in rows.chars(){
                let rand_num:f64 = rng.gen();
                if rand_num<0.05{
                    spaces.insert(Location{x:l.to_string(), y:l.to_string()}, LocationOccupancy::STAR);
                } else {
                    spaces.insert(Location{x:l.to_string(), y:n.to_string()}, LocationOccupancy::OPEN);
                }
            }
        }

        Board{spaces:spaces}

    }
    pub fn get_legal_moves(&self)->Vec<Location> {
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
        moves
    }
}

#[test]
fn test_board_creation(){
    let board = Board::new(42);

}

pub fn play_game(game_board: Board, players: Vec<Player>) {
    let mut players_turn:usize = 0;
    for _ in 0..48{
        let current_player = &players[players_turn];
        let legal_moves = game_board.get_legal_moves();
        println!("The moves are: {:?}", legal_moves);


        players_turn+=1;
        players_turn = players_turn % players.len();
    }

}

