use crate::{CompanyID, Location};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LocationOccupancy {
    OPEN,
    PLAYED,
    STAR,
    COMPANYID(CompanyID),
}

impl fmt::Display for LocationOccupancy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::OPEN => write!(f, "."),
            Self::PLAYED => write!(f, "+"),
            Self::STAR => write!(f, "*"),
            Self::COMPANYID(ref company_id) => fmt::Display::fmt(company_id, f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Moves(pub Vec<Location>);

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut comma_separated = String::new();

        for loc in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(format!("{}", &loc).as_str());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(format!("{}", &self.0[self.0.len() - 1]).as_str());
        write!(f, "{}", comma_separated)
    }
}

impl Moves {
    pub fn contains(&self, loc: &Location) -> bool {
        self.0.contains(loc)
    }
}
