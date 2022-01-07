use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum CompanyID {
    ALTAIR,
    BETELGEUSE,
    CAPELLA,
    DENEBOLA,
    ERIDANI,
}

impl CompanyID {
    pub fn name(&self) -> String {
        match &self {
            Self::ALTAIR => "ALTAIR STARWAYS".to_string(),
            Self::BETELGEUSE => "BETELGEUSE,LTD.".to_string(),
            Self::CAPELLA => "CAPELLA FREIGHT CO.".to_string(),
            Self::DENEBOLA => "DENEBOLA SHIPPERS".to_string(),
            Self::ERIDANI => "ERIDANI EXPEDITERS".to_string(),
        }
    }
}

impl fmt::Display for CompanyID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ALTAIR => write!(f, "A"),
            Self::BETELGEUSE => write!(f, "B"),
            Self::CAPELLA => write!(f, "C"),
            Self::DENEBOLA => write!(f, "D"),
            Self::ERIDANI => write!(f, "E"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Company {
    pub name: CompanyID,
    pub stock_price: i32,
}

impl Company {
    pub fn new(company_id: CompanyID) -> Company {
        Company {
            name: company_id,
            stock_price: 0,
        }
    }

    pub fn update_stock_price(&mut self, num_open_spaces: i32, num_star_spaces: i32) {
        self.stock_price += 500 * num_star_spaces as i32 + 100 * num_open_spaces as i32;
    }

    pub fn requires_split(&self) -> bool {
        self.stock_price > 3000
    }
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Company: {}\nStock Value: {}",
            self.name.name(),
            self.stock_price
        )
    }
}
