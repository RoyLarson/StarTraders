
use crate::{Board, Player};

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
