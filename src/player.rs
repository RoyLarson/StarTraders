use crate::CompanyID;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
    balance: u32,
    stocks: HashMap<CompanyID, usize>,
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
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name.as_str())
    }
}
