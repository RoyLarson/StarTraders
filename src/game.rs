use dialoguer::Input;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Board, Company, Location, LocationOccupancy, Moves, Player};

struct Merge {
    from: Company,
    to: Company,
}
pub enum PlayResult {
    Normal,
    NewCompany(Company),
    Merger(Merge),
}

pub fn play_game(mut board: Board, players: Vec<Player>) {
    let mut players_turn: usize = 0;
    for _ in 0..48 {
        print!("{}", &board);
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
        play_move(&mut board, &location);

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

pub fn play_move(board: &mut Board, location: &Location) -> PlayResult {
    let neighbors = board.location_neighbors(location);

    let is_next_to_company = neighbors
        .iter()
        .map(|loc| board.space(loc.clone()).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::COMPANY(_)));

    let is_next_to_star = neighbors
        .iter()
        .map(|loc| board.space(loc.clone()).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::STAR));

    let is_next_to_played = neighbors
        .iter()
        .map(|loc| board.space(loc.clone()).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::PLAYED));

    match (is_next_to_company, is_next_to_star, is_next_to_played) {
        (false, false, false) => {
            board.update_location(location.clone(), LocationOccupancy::PLAYED); // implements line 1230
            return PlayResult::Normal;
        }
        (false, false, true) => return create_and_set_company(board, location, neighbors),
        _ => panic!(),
    }
}

pub fn create_and_set_company(board: &mut Board, location: &Location) -> PlayResult {
    board.update_location(location.clone(), LocationOccupancy::PLAYED);

    match first_open_company(board) {
        Some(company) => {
            let occ = LocationOccupancy::COMPANY(company.clone());
            update_all_joined_locations(board, location, LocationOccupancy::PLAYED, &occ);
            return PlayResult::NewCompany(company.clone());
        }
        None => {}
    }

    PlayResult::Normal
}

fn update_all_joined_locations(
    board: &mut Board,
    location: &Location,
    to_update: LocationOccupancy,
    new_occ: &LocationOccupancy,
) {
    let mut to_visit: VecDeque<Location> = VecDeque::new();
    to_visit.push_front(location.clone());

    let mut visited = HashSet::new();

    while to_visit.len() > 0 {
        let loc = to_visit.pop_front().unwrap();
        visited.insert(loc.clone());
        to_visit.extend(
            board
                .location_neighbors(&loc)
                .iter()
                .filter(|loc| {
                    matches!(board.spaces.get(loc).unwrap(), to_update,) && !visited.contains(&loc)
                })
                .cloned()
                .collect::<VecDeque<Location>>(),
        );
        board.update_location(loc, new_occ.clone());
    }
}

fn first_open_company(board: &Board) -> Option<Company> {
    let company_occ = [
        LocationOccupancy::COMPANY(Company::ALTAIR),
        LocationOccupancy::COMPANY(Company::BETELGEUSE),
        LocationOccupancy::COMPANY(Company::CAPELLA),
        LocationOccupancy::COMPANY(Company::DENEBOLA),
        LocationOccupancy::COMPANY(Company::ERIDANI),
    ];

    let used_companies: HashSet<LocationOccupancy> = board
        .spaces
        .values()
        .filter(|occ| matches!(*occ, LocationOccupancy::COMPANY(_)))
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for comp in company_occ {
        if !used_companies.contains(&comp) {
            match comp {
                LocationOccupancy::COMPANY(name) => return Some(name),
                _ => panic!(format!("LocationOccupancy not correct: {:?}", comp)),
            }
        }
    }
    None
}
