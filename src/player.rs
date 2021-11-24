use std::collections::HashMap;
use crate::Company;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
    balance: usize,
    stocks: HashMap<Company,usize>,    
}

impl Player{
    pub fn new(name:String)->Player{
        let mut stocks = HashMap::new();
        stocks.insert(Company::ALTAIR,0);
        stocks.insert(Company::BETELGEUSE,0);
        stocks.insert(Company::CAPELLA,0);
        stocks.insert(Company::DENEBOLA,0);
        stocks.insert(Company::ERIDANI,0);
        Player{name, balance:100, stocks}

    }
}
