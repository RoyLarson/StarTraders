mod location;
mod game;
mod gameboard;

use std::collections::HashMap;



pub use location::Location;
pub use gameboard::{Board, Company, LocationOccupancy};





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








pub fn play_game(game_board: Board, players: Vec<Player>) {
    let mut players_turn:usize = 0;
    for _ in 0..48{
        let _current_player = &players[players_turn];
        let legal_moves = game_board.get_legal_moves();
        println!("The moves are: {}", legal_moves);


        players_turn+=1;
        players_turn %= players.len();
    }

}

