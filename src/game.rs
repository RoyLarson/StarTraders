use dialoguer::Input;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::{HashMap, HashSet};

use crate::{Board, Company, CompanyID, Location, LocationOccupancy, Moves, Player, Players};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Merge {
    company_a: CompanyID,
    company_b: CompanyID,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MergeResult {
    player_name: String,
    prior_merge_holdings: u32,
    prior_into_holdings: u32,
    new_stock_holdings: u32,
    bonus_paid: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AddSpaces {
    company_id: CompanyID,
    open: u32,
    stars: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PlayResult {
    NoCompanies,
    AddSpaces(AddSpaces),
    NewCompany(AddSpaces),
    Merger(AddSpaces, Merge),
}

pub fn play_game(mut board: Board, mut players: Players) {
    let mut companies = HashMap::<CompanyID, Company>::new();
    for i in 0..48 {
        print!("{}", &board);
        let num_players = players.len();
        let location = get_players_move(&board, &players, i);

        let result = play_move(&mut board, &location);

        match result {
            PlayResult::NoCompanies => {}
            PlayResult::AddSpaces(add) => {
                companies
                    .get_mut(&add.company_id)
                    .unwrap()
                    .update_stock_price(add.open, add.stars);
            }
            PlayResult::NewCompany(add) => {
                create_new_company(&add, &mut companies, &mut players.players[i % num_players])
            }
            PlayResult::Merger(add, merge) => {
                companies
                    .get_mut(&add.company_id)
                    .unwrap()
                    .update_stock_price(add.open, add.stars);
                merge_companies(merge, &location, &mut board, &mut players, &mut companies);
            }
        }
        if board
            .spaces
            .values()
            .filter(|occ| matches!(*occ, &LocationOccupancy::COMPANYID(_)))
            .count()
            > 0
        {}
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

fn create_new_company(
    add: &AddSpaces,
    companies: &mut HashMap<CompanyID, Company>,
    player: &mut Player,
) {
    println!("{:^1$}", "!!! SPECIAL ANNOUNCEMENT !!!", 37);
    println!();
    println!("A NEW COMPANY HAS BEN FORMED:\n{}", add.company_id.name());
    println!();
    companies.insert(add.company_id.clone(), Company::new(add.company_id.clone()));
    companies
        .get_mut(&add.company_id)
        .unwrap()
        .update_stock_price(add.open, add.stars);
    println!("{}", companies.get(&add.company_id).unwrap());

    player.update_stock(&add.company_id, 5);
    println!("{}", player)
}

fn get_players_move(board: &Board, players: &Players, i: usize) -> Location {
    let mut legal_moves = get_legal_moves(&board);
    legal_moves.sort();
    let current_player = &players[i % players.len()];

    println!("{}\n", current_player);
    println!("The moves are: {}\n", legal_moves);
    let location = Input::<Location>::new()
        .with_prompt(format!("{} what is your move", &current_player.name))
        .validate_with(|input: &Location| -> Result<(), &str> {
            if legal_moves.contains(input) {
                Ok(())
            } else {
                Err("That is not a legal move")
            }
        })
        .interact()
        .unwrap();
    location
}

fn get_legal_moves(board: &Board) -> Moves {
    let open_locations: Vec<Location> = board
        .spaces
        .keys()
        .filter(|loc| legal_move(board, loc))
        .choose_multiple(&mut rand::thread_rng(), 6)
        .into_iter()
        .cloned()
        .collect();

    Moves::new(open_locations)
}

fn legal_move(board: &Board, location: &Location) -> bool {
    let occupancy = *board.spaces.get(location).unwrap();

    if occupancy != LocationOccupancy::OPEN {
        // implements line 670 in trade.bas
        return false;
    }

    // REMOVED BECAUSE THE THE LAST LOGIC
    // DOES NOT ALLOW FOR VERY MANY PLACES
    // // implements lines 680, 690, 700
    // let has_companies = board
    //     .spaces
    //     .values()
    //     .any(|occ| matches!(occ, LocationOccupancy::COMPANY(_)));

    // if !has_companies {
    //     return true;
    // }

    // let neighbors: Vec<LocationOccupancy> = board
    //     .location_neighbors(&location)
    //     .into_iter()
    //     .map(|loc| board.spaces.get(&loc).unwrap())
    //     .cloned()
    //     .collect();

    // let mut neighbor_counts: HashMap<LocationOccupancy, usize> = HashMap::new();

    // for occupancy in neighbors {
    //     *neighbor_counts.entry(occupancy).or_insert(0) += 1;
    // }

    // let company_counts = neighbor_counts
    //     .iter()
    //     .filter(|(occ, count)| matches!(*occ, LocationOccupancy::COMPANY(_)) && *count > &0)
    //     .collect::<Vec<(&LocationOccupancy, &usize)>>()
    //     .iter()
    //     .count();

    // // implements lines 710, 720, 730, 740
    // if company_counts > 0 {
    //     return true;
    // }

    // // implements lines 750 - 860
    // let next_to_played = neighbor_counts.get(&LocationOccupancy::PLAYED).is_some();
    // let next_to_star = neighbor_counts.get(&LocationOccupancy::STAR).is_some();

    // TODO: this line here keeps new companies from forming too often
    // if (next_to_played || next_to_star) && (company_counts < 3) {
    //     return false;
    // }

    true
}

fn play_move(board: &mut Board, location: &Location) -> PlayResult {
    let neighbors = board.location_neighbors(location);

    let is_next_to_company = neighbors
        .iter()
        .map(|loc| board.spaces.get(loc).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::COMPANYID(_)));

    let is_next_to_star = neighbors
        .iter()
        .map(|loc| board.space(loc).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::STAR));

    let is_next_to_played = neighbors
        .iter()
        .map(|loc| board.space(loc).unwrap())
        .any(|occ| matches!(occ, LocationOccupancy::PLAYED));

    match (is_next_to_company, is_next_to_star, is_next_to_played) {
        (false, false, false) => {
            board.update_location(location.clone(), LocationOccupancy::PLAYED); // implements line 1230
        }
        (false, _, true) => match determine_new_company(board) {
            Some(company) => {
                let occ = LocationOccupancy::COMPANYID(company);
                let (open, stars) =
                    board.update_all_joined_locations(location, LocationOccupancy::PLAYED, occ);
                return PlayResult::NewCompany(AddSpaces {
                    company_id: company,
                    open,
                    stars,
                });
            }
            None => board.update_location(location.clone(), LocationOccupancy::PLAYED),
        },
        (false, true, _) => match determine_new_company(board) {
            Some(company) => {
                let occ = LocationOccupancy::COMPANYID(company);
                let (open, stars) =
                    board.update_all_joined_locations(location, LocationOccupancy::PLAYED, occ);
                return PlayResult::NewCompany(AddSpaces {
                    company_id: company,
                    open,
                    stars,
                });
            }
            None => board.update_location(location.clone(), LocationOccupancy::PLAYED),
        },
        (true, _, _) => match requires_merger(board, location) {
            companies if companies.is_empty() => {
                panic!("No companies returned from requires_merger")
            }
            companies if companies.len() == 1 => {
                let company = companies.iter().cloned().collect::<Vec<CompanyID>>()[0];
                let (open, stars) = board.update_all_joined_locations(
                    location,
                    LocationOccupancy::PLAYED,
                    LocationOccupancy::COMPANYID(company),
                );
                return PlayResult::AddSpaces(AddSpaces {
                    company_id: company,
                    open,
                    stars,
                });
            }
            mut companies if companies.len() == 2 => {
                let company_a = *companies.iter().next().unwrap();
                companies.remove(&company_a);
                let company_b = *companies.iter().next().unwrap();
                companies.remove(&company_b);

                let merge = Merge {
                    company_a: company_b,
                    company_b: company_a,
                };

                let (open, stars) = board.update_all_joined_locations(
                    location,
                    LocationOccupancy::PLAYED,
                    LocationOccupancy::COMPANYID(merge.company_b),
                );

                return PlayResult::Merger(
                    AddSpaces {
                        company_id: merge.company_b,
                        open,
                        stars,
                    },
                    merge,
                );
            }
            companies => panic!(
                "More than 2 companies returned from requires_merger {:?}",
                companies
            ),
        },
    }
    PlayResult::NoCompanies
}

fn determine_new_company(board: &Board) -> Option<CompanyID> {
    let company_occ = [
        LocationOccupancy::COMPANYID(CompanyID::ALTAIR),
        LocationOccupancy::COMPANYID(CompanyID::BETELGEUSE),
        LocationOccupancy::COMPANYID(CompanyID::CAPELLA),
        LocationOccupancy::COMPANYID(CompanyID::DENEBOLA),
        LocationOccupancy::COMPANYID(CompanyID::ERIDANI),
    ];

    let used_companies: HashSet<LocationOccupancy> = board
        .spaces
        .values()
        .filter(|occ| matches!(*occ, LocationOccupancy::COMPANYID(_)))
        .cloned()
        .collect::<HashSet<_>>();

    for occ in company_occ {
        if !used_companies.contains(&occ) {
            match occ {
                LocationOccupancy::COMPANYID(name) => return Some(name),
                _ => panic!("LocationOccupancy not of COMPANY: {:?}", occ),
            }
        }
    }
    None
}

fn requires_merger(board: &Board, location: &Location) -> HashSet<CompanyID> {
    let neighbor_companies = board
        .location_neighbors(location)
        .iter()
        .filter(|loc| {
            matches!(
                board.spaces.get(*loc).unwrap(),
                LocationOccupancy::COMPANYID(_)
            )
        })
        .map(|loc| match *board.spaces.get(loc).unwrap() {
            LocationOccupancy::COMPANYID(company) => company,
            _ => panic!("WTF the filter in requires_merger didn't work"),
        })
        .collect::<HashSet<_>>();

    neighbor_companies
}

fn merge_companies(
    merge: Merge,
    location: &Location,
    board: &mut Board,
    players: &mut Players,
    companies: &mut HashMap<CompanyID, Company>,
) -> Vec<MergeResult> {
    println!("Merging: {:?}", merge);
    let company_a = companies.get(&merge.company_a).unwrap().clone();
    let company_b = companies.get(&merge.company_b).unwrap().clone();
    let acquired_id;
    let acquired;
    let acquirer_id;

    if company_a.stock_price > company_b.stock_price {
        companies.remove_entry(&merge.company_b).unwrap();
        acquired_id = &merge.company_b;
        acquired = company_b;
        acquirer_id = &merge.company_a;
    } else {
        companies.remove_entry(&merge.company_a).unwrap();
        acquired_id = &merge.company_a;
        acquired = company_a;
        acquirer_id = &merge.company_b;
    }

    println!(
        "Company: {:?}\nAcquirer: {:?}\nAcquired: {:?}\nAcquirer in Companies: {:?}\nAcquired in Companies: {:?}",
        companies,
        acquirer_id,
        acquired_id,
        companies.contains_key(acquirer_id),
        companies.contains_key(&acquired_id)
    );

    let mut results = Vec::<MergeResult>::new();
    let total_stock_held_of_merge_company: u32 = players
        .iter()
        .map(|player| player.get_stock(&acquired_id))
        .sum();

    let merge_stock_price = acquired.stock_price as u32;

    companies.get_mut(acquirer_id).unwrap().stock_price += acquired.stock_price;

    for player in players.iter_mut() {
        let merge_stock = player.get_stock(&acquired_id);
        let into_stock = player.get_stock(&acquirer_id);
        let converted_stock = (0.5 * merge_stock as f32 + 0.5).round() as u32;
        let total_holdings = converted_stock + player.get_stock(&acquirer_id);
        let player_bonus = (10.0
            * (merge_stock as f32 / total_stock_held_of_merge_company as f32)
            * (merge_stock_price as f32))
            .round() as i32;

        player.update_stock(&acquired_id, 0);
        player.update_stock(&acquirer_id, total_holdings);
        player.update_balance(player_bonus).unwrap();

        results.push(MergeResult {
            player_name: player.name.clone(),
            prior_merge_holdings: merge_stock,
            prior_into_holdings: into_stock,
            new_stock_holdings: total_holdings,
            bonus_paid: player_bonus,
        })
    }
    board.update_all_joined_locations(
        location,
        LocationOccupancy::COMPANYID(*acquired_id),
        LocationOccupancy::COMPANYID(*acquirer_id),
    );

    results
}
