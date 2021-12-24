use dialoguer::Input;
use rand::prelude::*;

use startraders::{play_game, query_to_display_instructions, setup_board, Board, Player};

fn main() {
    println!("              **********   STAR TRADERS   **********");
    query_to_display_instructions();

    let mut players: Vec<Player> = Vec::new();
    let num_players: u32 = Input::new()
        .with_prompt("How many players are playing:")
        .default(1)
        .interact()
        .unwrap();

    for i in 0..num_players {
        let name = Input::<String>::new()
            .with_prompt(format!("What is player {}'s name", i + 1))
            .interact()
            .unwrap();
        let player = Player::new(name);
        players.push(player);
    }
    println!("I will now shuffle the players");
    let mut rng = rand::thread_rng();
    players.shuffle(&mut rng);
    println!("The player order is {:?}", players);
    let board_seed = Input::<usize>::new()
        .with_prompt("Input a game board number")
        .interact()
        .unwrap();

    let mut game_board = Board::new();
    setup_board(&mut game_board, board_seed);
    println!("{}", game_board);
    play_game(game_board, players);
}
