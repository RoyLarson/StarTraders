use crate::CompanyID;
use std::collections::HashMap;
use std::fmt;
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    balance: i32,
    stocks: HashMap<CompanyID, i32>,
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
    pub fn add_stock(&mut self, company_id: &CompanyID, amount: i32) {
        *self.stocks.entry(*company_id).or_insert(0) += amount;
    }

    pub fn get_stock(&self, company_id: &CompanyID) -> i32 {
        match self.stocks.get(company_id) {
            Some(x) => *x,
            None => 0,
        }
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn update_balance(&mut self, delta: i32) -> Result<(), &str> {
        let new_balance = self.balance + delta;
        if new_balance >= 0 {
            self.balance = new_balance;
            return Ok(());
        }
        Err("Insufficient Funds")
    }

    pub fn update_stock(&mut self, company_id: &CompanyID, new_value: i32) {
        *self.stocks.entry(*company_id).or_insert(0) = new_value;
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

    pub fn iter(&self) -> std::slice::Iter<'_, Player> {
        self.players.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Player> {
        self.players.iter_mut()
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

impl IntoIterator for Players {
    type Item = Player;
    type IntoIter = <Vec<Player> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.players.into_iter()
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Player;
    type IntoIter = <&'a Vec<Player> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.players.iter()
    }
}

impl<'a> IntoIterator for &'a mut Players {
    type Item = &'a mut Player;
    type IntoIter = <&'a mut Vec<Player> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.players.iter_mut()
    }
}
