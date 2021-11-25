use crate::{Board, Player, Location};
use dialoguer::Input;

pub fn play_game(game_board: Board, players: Vec<Player>) {
    let mut players_turn:usize = 0;
    for _ in 0..48{
        let current_player = &players[players_turn];
        let legal_moves = game_board.get_legal_moves();
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
