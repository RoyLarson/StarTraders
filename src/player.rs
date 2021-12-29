use crate::CompanyID;
use std::collections::HashMap;
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    balance: u32,
    stocks: HashMap<CompanyID, u32>,
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut stocks = HashMap::new();
        stocks.insert(CompanyID::ALTAIR, 0);
        stocks.insert(CompanyID::BETELGEUSE, 0);
        stocks.insert(CompanyID::CAPELLA, 0);
        stocks.insert(CompanyID::DENEBOLA, 0);
        stocks.insert(CompanyID::ERIDANI, 0);
        Player {
            name,
            balance: 6000,
            stocks,
        }
    }
    pub fn add_stock(&mut self, company_id: &CompanyID, amount: u32) {
        *self.stocks.entry(*company_id).or_insert(0) += amount;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Player: {}\nBalance: {}\nStocks: {:?}",
            &self.name.as_str(),
            self.balance,
            self.stocks
        )
    }
}

pub struct Players {
    pub players: Vec<Player>,
}

impl Players {
    pub fn new() -> Players {
        Players {
            players: Vec::new(),
        }
    }
    pub fn from_vec(players: Vec<Player>) -> Players {
        Players { players }
    }
    pub fn push(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn len(&self) -> usize {
        self.players.len()
    }
    pub fn is_empty(&self) -> bool {
        self.players.len() == 0
    }
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        let mut player_iter = self.players.iter();
        result.push_str(format!("{}", player_iter.next().unwrap()).as_str());

        for player in player_iter {
            result.push('\n');
            result.push_str(format!("{}", player).as_str());
        }

        write!(f, "{}", result)
    }
}

impl Default for Players {
    fn default() -> Self {
        Players::new()
    }
}

impl Index<usize> for Players {
    type Output = Player;
    fn index(&self, i: usize) -> &Player {
        if i >= self.players.len() {
            panic!("Too Far")
        }
        &self.players[i]
    }
}

impl IndexMut<usize> for Players {
    fn index_mut(&mut self, i: usize) -> &mut Player {
        &mut self.players[i]
    }
}
