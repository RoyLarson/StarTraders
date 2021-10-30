use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Company{
    ALTAIR,
    BETELGEUSE,
    CAPELLA,
    DENEBOLA,
    ERIDANI,
}

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
        Player{name:name, balance:0, stocks:stocks}

    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location{
    x: usize,
    y: usize
}

// pub struct Board {
//     company_loc: HashMap<
// }

// impl Board {
//     pub fn new()->Board {

//     }
// }