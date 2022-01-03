use dialoguer::Input;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Board, Company, CompanyID, Location, LocationOccupancy, Moves, Player, Players};

struct Merge {
    acquired: CompanyID,
    acquirer: CompanyID,
}

struct MergeResult {
    player_name: String,
    prior_merge_holdings: u32,
    prior_into_holdings: u32,
    new_stock_holdings: u32,
    bonus_paid: i32,
}

struct AddSpaces {
    company_id: CompanyID,
    open: u32,
    stars: u32,
}
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
    println!("{:^1$}", "SPECIAL ANNOUNCEMENT !!!", 37);
    println!();
    println!("A NEW COMPANY HAS BEN FORMED: {}", add.company_id.name());
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
        (false, _, true) => {
            if let Some(company) = determine_new_company(board) {
                let occ = LocationOccupancy::COMPANYID(company);
                let (open, stars) =
                    update_all_joined_locations(board, location, LocationOccupancy::PLAYED, occ);
                return PlayResult::NewCompany(AddSpaces {
                    company_id: company,
                    open,
                    stars,
                });
            }
        }
        (false, true, _) => match determine_new_company(board) {
            Some(company) => {
                let occ = LocationOccupancy::COMPANYID(company);
                let (open, stars) =
                    update_all_joined_locations(board, location, LocationOccupancy::PLAYED, occ);
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
                let (open, stars) = update_all_joined_locations(
                    board,
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

                let a_size = board
                    .spaces
                    .values()
                    .filter(|occ| matches!(*occ, LocationOccupancy::COMPANYID(_company_a)))
                    .count();

                let b_size = board
                    .spaces
                    .values()
                    .filter(|occ| matches!(*occ, LocationOccupancy::COMPANYID(_company_b)))
                    .count();

                let mut merge = Merge {
                    acquired: company_b,
                    acquirer: company_a,
                };

                if b_size > a_size {
                    merge = Merge {
                        acquired: company_a,
                        acquirer: company_b,
                    };
                }

                let (open, stars) = update_all_joined_locations(
                    board,
                    location,
                    LocationOccupancy::PLAYED,
                    LocationOccupancy::COMPANYID(merge.acquirer),
                );

                return PlayResult::Merger(
                    AddSpaces {
                        company_id: merge.acquirer,
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

fn update_all_joined_locations(
    board: &mut Board,
    location: &Location,
    to_update: LocationOccupancy,
    new_occ: LocationOccupancy,
) -> (u32, u32) {
    let mut to_visit: VecDeque<Location> = VecDeque::new();
    let mut count_open = 0 as u32;
    let mut count_stars = 0 as u32;
    to_visit.push_front(location.clone());

    let mut visited = HashSet::new();

    while !to_visit.is_empty() {
        let loc = to_visit.pop_front().unwrap();
        visited.insert(loc.clone());
        to_visit.extend(
            board
                .location_neighbors(&loc)
                .iter()
                .filter(|loc| {
                    (*board.spaces.get(loc).unwrap() == to_update) && !visited.contains(loc)
                })
                .cloned()
                .collect::<VecDeque<Location>>(),
        );
        board.update_location(loc.clone(), new_occ);
        count_open += 1;
        count_stars += board
            .location_neighbors(&loc)
            .iter()
            .filter(|loc| matches!(board.spaces.get(loc).unwrap(), LocationOccupancy::STAR))
            .count() as u32;
    }
    (count_open, count_stars)
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
    update_all_joined_locations(
        board,
        location,
        LocationOccupancy::COMPANYID(merge.acquired),
        LocationOccupancy::COMPANYID(merge.acquirer),
    );
    let (acquired_id, acquired) = companies.remove_entry(&merge.acquired).unwrap();
    let acquirer_id = merge.acquirer;
    let mut results = Vec::<MergeResult>::new();
    let total_stock_held_of_merge_company: u32 = players
        .iter()
        .map(|player| player.get_stock(&merge.acquired))
        .sum();

    let merge_stock_price = companies.get(&acquired_id).unwrap().stock_price as u32;

    companies.get_mut(&acquirer_id).unwrap().stock_price += acquired.stock_price;

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
    companies.insert(acquired_id, Company::new(acquired_id));
    results
}
