use std::collections::HashMap;
use std::borrow::Cow;
use rand::prelude::*;
use rand_pcg::Pcg64;



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
        Player{name:name, balance:100, stocks:stocks}

    }
}

pub enum LocationOccupancy{
    OPEN,
    STAR,
    COMPANY(Company)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location{
    pub x: char,
    pub y: char
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
                    spaces.insert(Location{x:l, y:n}, LocationOccupancy::STAR);
                } else {
                    spaces.insert(Location{x:l, y:n}, LocationOccupancy::OPEN);
                }
            }
        }

        Board{spaces:spaces}

    }
    pub fn get_legal_moves(&self)->Vec<&Location>{
        let mut open_locations= Vec::<Location>::new();
        for (location, occupation) in &self.spaces{
            match occupation{
                LocationOccupancy::OPEN=>{
                    open_locations.push(location.clone());
                }
                _=>{}
            }
        }
        let locations: Vec<_> = open_locations.choose_multiple(&mut rand::thread_rng(), 5).collect();
        locations.copy()
    }
}

#[test]
fn test_board_creation(){
    let board = Board::new(42);

}

pub fn play_game(game_board: Board, players: Vec<Player>) {
    let mut players_turn:usize = 0;
    for _ in 0..48{
        let legal_moves = game_board.get_legal_moves();
        println!("The moves are: {:?}", legal_moves);
    }
    let current_player = &players[players_turn];

}

