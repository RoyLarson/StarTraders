use dialoguer::Input;
use rand::prelude::*;

use startraders::{Player, Board, play_game, setup_board, query_to_display_instructions};

fn main() {
    println!("              **********   STAR TRADERS   **********");
    query_to_display_instructions();

    let mut players:Vec<Player> = Vec::new();
    let num_players:u32 = Input::new()
        .with_prompt("How many players are playing:")
        .default(1)
        .interact()
        .unwrap();


    for i in 0..num_players{
        let name = Input::<String>::new()
            .with_prompt(format!("What is player {}'s name",i+1))
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
    let mut game_board = setup_board(Board::new(), board_seed);
    play_game(game_board, players);

}
