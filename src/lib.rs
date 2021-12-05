mod location;
mod game;
mod gameboard;
mod player;
mod instructions;

pub use location::Location;
pub use gameboard::{Board, Company, LocationOccupancy, Moves};
pub use player::Player;
pub use game::{play_game, setup_board};
pub use instructions::{query_to_display_instructions,print_instructions};














