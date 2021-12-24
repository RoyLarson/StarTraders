use dialoguer::Input;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::HashMap;

use crate::{Board, Location, LocationOccupancy, Moves, Player};

pub fn play_game(mut board: Board, players: Vec<Player>) {
    let mut players_turn: usize = 0;
    for _ in 0..48 {
        let current_player = &players[players_turn];
        let legal_moves = get_legal_moves(&board);
        println!("The moves are: {}", legal_moves);
        let location = Input::<Location>::new()
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

        players_turn += 1;
        players_turn %= players.len();
    }
}

pub fn setup_board(board: &mut Board, seed: usize) {
    let mut rng = Pcg64::seed_from_u64(seed as u64);

    let locations = board.get_spaces();
    for location in locations.iter() {
        let rand_num: f64 = rng.gen();
        if rand_num < 0.05 {
            board.update_location(location.clone(), LocationOccupancy::STAR);
        }
    }
}

pub fn get_legal_moves(board: &Board) -> Moves {
    let open_locations: Vec<Location> = board
        .spaces
        .keys()
        .filter(|loc| legal_move(&board, loc))
        .choose_multiple(&mut rand::thread_rng(), 6)
        .into_iter()
        .cloned()
        .collect();

    Moves(open_locations)
}

pub fn legal_move(board: &Board, location: &Location) -> bool {
    let occupancy = board.spaces.get(&location).unwrap().clone();

    if occupancy != LocationOccupancy::OPEN {
        // implements line 670 in trade.bas
        return false;
    }

    // implements lines 680, 690, 700
    let has_companies = board
        .spaces
        .values()
        .any(|occ| matches!(occ, LocationOccupancy::COMPANY(_)));

    if !has_companies {
        return true;
    }

    let neighbors: Vec<LocationOccupancy> = board
        .location_neighbors(&location)
        .into_iter()
        .map(|loc| board.spaces.get(&loc).unwrap())
        .cloned()
        .collect();

    let mut neighbor_counts: HashMap<LocationOccupancy, usize> = HashMap::new();

    for occupancy in neighbors {
        *neighbor_counts.entry(occupancy).or_insert(0) += 1;
    }

    let company_counts = neighbor_counts
        .iter()
        .filter(|(occ, count)| matches!(*occ, LocationOccupancy::COMPANY(_)) && *count > &0)
        .collect::<Vec<(&LocationOccupancy, &usize)>>()
        .iter()
        .count();

    // implements lines 710, 720, 730, 740
    if company_counts > 0 {
        return true;
    }

    // implements lines 750 - 860
    let next_to_played = neighbor_counts.get(&LocationOccupancy::PLAYED).is_some();
    let next_to_star = neighbor_counts.get(&LocationOccupancy::STAR).is_some();

    if (next_to_played || next_to_star) && (company_counts < 3) {
        return false;
    }

    true
}
