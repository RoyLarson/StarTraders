mod instructions;

use instructions::query_to_display_instructions;
use std::io;
use startraders::Player;
use dialoguer::Input;

fn main() {
    println!("              **********   STAR TRADERS   **********");
    query_to_display_instructions();

    let mut players:Vec<Player> = Vec::new();
    let mut player:Player;
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
    println!("Players: {:?}", players)
}
