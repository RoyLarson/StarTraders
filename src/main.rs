mod instructions;
// mod players;
// mod game_board;

use instructions::query_to_display_instructions;
use std::io;
use startraders::Player;


fn main() {
    println!("              **********   STAR TRADERS   **********");
    query_to_display_instructions();
    let player = Player::new("Roy".to_string());
    println!("Player: {:?}", player);

}
