mod instructions;
// mod players;
// mod game_board;

use instructions::query_to_display_instructions;
use std::io;
use startraders::Player;


fn main() {
    println!("              **********   STAR TRADERS   **********");
    query_to_display_instructions();
    let mut input = String::new();
    let mut players:Vec<Player> = Vec::new();
    let mut player:Player;
    let mut num_players:u32 = 0;

    println!("How many people are playing?");
    loop {
        io::stdin().read_line(&mut input).unwrap();
        input = input.chars().filter(|c| !c.is_whitespace()).collect();
        num_players = match input.parse::<u32>(){
            Ok(n) => n,
            Err(_) => 0
        };

        if (0<num_players) && (num_players<=4) {
            break
        } 
        println!("Num players need to be between 1 and 4: input={}", &num_players);
    }

    for i in 0..num_players{
        println!("Please input players {} name:", i+1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let player = Player::new(input);
        players.push(player);
    }
    println!("Players: {:?}", players)
}
