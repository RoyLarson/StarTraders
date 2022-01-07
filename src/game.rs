use dialoguer::Input;
use itertools::sorted;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeBounds,
};

use crate::{Board, Company, CompanyID, Location, LocationOccupancy, Moves, Player, Players};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Merge {
    company_a: CompanyID,
    company_b: CompanyID,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MergeResult {
    player_name: String,
    old_stock: i32,
    new_stock: i32,
    total_holdings: i32,
    bonus_paid: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AddSpaces {
    company_id: CompanyID,
    open: i32,
    stars: i32,
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
    let num_players = players.len();
    for i in 0..48 {
        let location;
        print!("{}", &board);
        {
            let current_player = &players.players[i % num_players];
            location = get_players_move(&board, current_player);
        }
        match play_move(&mut board, &location) {
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
                let result =
                    merge_companies(merge, &location, &mut board, &mut players, &mut companies);
                player_merge_results(result);
            }
        }

        for (company_id, company) in companies.iter_mut() {
            if company.requires_split() {
                split_stock(company_id, company, &mut players);
            }
        }
        player_dividends_and_stock_purchase(&companies, &mut players[i % num_players])
    }
    println!("!!! END OF GAME !!!");
    for player in &players {
        let mut net_worth = player.get_balance();
        for (company_id, company) in &companies {
            net_worth += player.get_stock(company_id) * company.stock_price;
        }
        println!("PLAYER: {} NET WORTH: {}", player.name, net_worth);
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
    companies.insert(add.company_id, Company::new(add.company_id));
    companies
        .get_mut(&add.company_id)
        .unwrap()
        .update_stock_price(add.open, add.stars);
    println!("{}", companies.get(&add.company_id).unwrap());

    player.update_stock(&add.company_id, 5);
    println!("{}", player)
}

fn get_players_move(board: &Board, current_player: &Player) -> Location {
    let mut legal_moves = get_legal_moves(board);
    legal_moves.sort();

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

    let loc_neighbors = board.location_neighbors(location);
    let loc_neighbors_occ = loc_neighbors
        .iter()
        .map(|loc| *board.space(loc).unwrap())
        .collect::<Vec<LocationOccupancy>>();

    let num_companies = board
        .spaces
        .iter()
        .filter(|(_, occ)| matches!(*occ, LocationOccupancy::COMPANYID(_)))
        .map(|(_, occ)| occ)
        .collect::<HashSet<_>>()
        .len();

    let num_company_types = 5;

    let init_new_company = (loc_neighbors_occ.contains(&LocationOccupancy::STAR)
        | loc_neighbors_occ.contains(&LocationOccupancy::PLAYED))
        & (!loc_neighbors_occ
            .iter()
            .any(|occ| matches!(occ, LocationOccupancy::COMPANYID(_))));

    // Check if all companies are used
    // if all companies are in play then check
    // if the play would normally cause another company
    // then it should not be allowed

    if (num_companies == num_company_types) & init_new_company {
        return false;
    }

    let num_neighbor_company_types = loc_neighbors_occ
        .iter()
        .filter(|occ| matches!(occ, LocationOccupancy::COMPANYID(_)))
        .collect::<HashSet<_>>()
        .len();

    // If because I don't have the logic to merge
    // 3 or more companies don't allow those positions
    if num_neighbor_company_types > 3 {
        return false;
    }

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
    let company_a = *companies.get(&merge.company_a).unwrap();
    let company_b = *companies.get(&merge.company_b).unwrap();
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

    let mut results = Vec::<MergeResult>::new();
    let total_stock_held_of_merge_company: i32 = players
        .iter()
        .map(|player| player.get_stock(acquired_id))
        .sum();

    let merge_stock_price = acquired.stock_price as i32;

    companies.get_mut(acquirer_id).unwrap().stock_price += acquired.stock_price;

    for player in players.iter_mut() {
        let merge_stock = player.get_stock(acquired_id);
        let into_stock = player.get_stock(acquirer_id);
        let converted_stock = (0.5 * merge_stock as f32 + 0.5).round() as i32;
        let total_holdings = converted_stock + player.get_stock(acquirer_id);
        let player_bonus = (10.0
            * (merge_stock as f32 / total_stock_held_of_merge_company as f32)
            * (merge_stock_price as f32))
            .round() as i32;

        player.update_stock(acquired_id, 0);
        player.update_stock(acquirer_id, total_holdings);
        player.update_balance(player_bonus).unwrap();

        results.push(MergeResult {
            player_name: player.name.clone(),
            old_stock: merge_stock,
            new_stock: into_stock,
            total_holdings,
            bonus_paid: player_bonus,
        })
    }
    board.update_all_joined_locations(
        location,
        LocationOccupancy::COMPANYID(*acquired_id),
        LocationOccupancy::COMPANYID(*acquirer_id),
    );

    println!("{:^1$}", "!!! SPECIAL ANNOUNCEMENT !!!", 37);
    println!();
    println!(
        "{} HAS BEEN MERGED INTO {}",
        acquired_id.name(),
        acquirer_id.name()
    );

    results
}

fn player_merge_results(results: Vec<MergeResult>) {
    for result in results {
        println!(
            "PLAYER: {}\n\tOLD STOCK: {}\n\tNEW STOCK: {}\n\tTOTAL HOLDINGS: {}\n\tBONUS PAID: {}",
            result.player_name,
            result.old_stock,
            result.new_stock,
            result.total_holdings,
            result.bonus_paid
        )
    }
}

fn split_stock(company_id: &CompanyID, company: &mut Company, players: &mut Players) {
    let prior_stock_price = company.stock_price as i32;
    let new_stock_price = company.stock_price as i32 / 2_i32;

    println!("{:^1$}", "!!! SPECIAL ANNOUNCEMENT !!!", 37);
    println!();
    println!("THE STOCK OF {} HAS SPLIT 2 FOR 1", company_id.name());
    println!(
        "ORIGINAL VALUE: {} NEW VALUE: {}",
        &prior_stock_price, &new_stock_price
    );

    for player in players {
        let orig_stock = player.get_stock(company_id) as i32;
        let old_value = prior_stock_price * orig_stock;
        let new_stock = (old_value as f32 / new_stock_price as f32).ceil() as i32;

        player.add_stock(company_id, (new_stock - orig_stock) as i32);
        println!(
            "PLAYER: {} PREVIOUS HOLDINGS: {} NEW HOLDINGS: {}",
            player.name,
            orig_stock,
            player.get_stock(company_id)
        );
    }
    company.stock_price = new_stock_price;
}

fn player_dividends_and_stock_purchase(
    companies: &HashMap<CompanyID, Company>,
    current_player: &mut Player,
) {
    // Calculate dividends from owned stock
    for (company_id, company) in companies {
        current_player
            .update_balance(
                (0.05 * current_player.get_stock(company_id) as f32 * company.stock_price as f32)
                    .round() as i32,
            )
            .expect("The current player could not add dividends");
    }

    // Allow for purchase of stock
    for (company_id, company) in sorted(companies) {
        println!(
            "PLAYER: {} CURRENT BALANCE: {}",
            current_player.name,
            current_player.get_balance()
        );
        let stocks_to_purchase = Input::<u32>::new()
            .default(0)
            .with_prompt(format!(
                "HOW MANY SHARES OF {} DO YOU WANT TO PURCHASE?\nCURRENT PRICE: {} MAX SHARES: {}",
                company_id.name(),
                company.stock_price,
                current_player.get_balance() / company.stock_price
            ))
            .validate_with(|input: &u32| -> Result<(), _> {
                match current_player.get_balance() >= (*input * company.stock_price as u32) as i32 {
                    true => Ok(()),
                    false => Err(format!(
                        "Insufficient Funds: {} < {}",
                        current_player.get_balance(),
                        input * company.stock_price as u32,
                    )),
                }
            })
            .interact()
            .unwrap();

        let total_purchase: i32 = stocks_to_purchase as i32 * company.stock_price;

        current_player.add_stock(company_id, stocks_to_purchase as i32);
        current_player
            .update_balance(-total_purchase)
            .expect("Something in Roy's validator didn't work");
    }
}
