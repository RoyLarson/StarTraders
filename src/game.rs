use dialoguer::Input;
use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::{Board, Player, Location, Moves, LocationOccupancy};


pub fn play_game(board: Board, players: Vec<Player>) {
    let mut players_turn:usize = 0;
    for _ in 0..48{
        let current_player = &players[players_turn];
        let legal_moves = get_legal_moves(&board);
        println!("The moves are: {}", legal_moves);
        let location= Input::<Location>::new()
        .with_prompt(format!("Player: {} what is your move", &current_player))
        .validate_with(|input: &Location| -> Result<(), &str> {
            if legal_moves.contains(input) {
                Ok(())
            } else {
                Err("That is not a legal move")
            }
        })
        .interact()
        .unwrap();

        players_turn+=1;
        players_turn %= players.len();
    }

}

pub fn setup_board(mut board:Board, seed:usize)->Board {
    let mut rng= Pcg64::seed_from_u64(seed as u64);

    let locations = board.get_spaces();
    for location in locations.iter() {
        let rand_num:f64 = rng.gen();   
        if rand_num<0.05{
            board.update_location(location.to_owned(), LocationOccupancy::STAR);
        }
    }
    board
}

pub fn get_legal_moves(board: &Board) -> Moves {
    let mut open_locations= Vec::<Location>::new();
    for (location, occupation) in &board.spaces{
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