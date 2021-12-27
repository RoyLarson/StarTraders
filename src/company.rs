#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
