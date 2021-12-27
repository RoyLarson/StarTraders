mod game;
mod gameboard;
mod gamepieces;
mod instructions;
mod location;
mod player;

pub use game::{play_game, setup_board};
pub use gameboard::Board;
pub use gamepieces::{CompanyID, LocationOccupancy, Moves};
pub use instructions::{print_instructions, query_to_display_instructions};
pub use location::Location;
pub use player::Player;
