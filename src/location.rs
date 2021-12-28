use std::cmp;

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, Eq, PartialOrd, Ord)]
pub struct Location {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone)]
pub enum ParsePointError {
    FailedParse(String),
    Not2Dimensional(usize),
    NonNumeric,
}

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Location {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s
            .trim_matches(|p| p == '(' || p == ')')
            .trim()
            .replace(|p| p == ' ', "");
        {
            if !clean_s
                .contains(|c| ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&c))
            {
                return Err(ParsePointError::NonNumeric);
            }
        }
        let coords: String = clean_s.chars().collect::<String>();

        if coords.len() != 2 {
            return Err(ParsePointError::Not2Dimensional(coords.len()));
        }
        let x = Some(match &coords.chars().next() {
            Some(c) => c.to_string(),
            None => {
                return Err(ParsePointError::FailedParse(
                    "X value is not a char ".to_string(),
                ))
            }
        });

        let y = Some(match &coords.chars().nth(1) {
            Some(c) => c.to_string(),
            None => {
                return Err(ParsePointError::FailedParse(
                    "Y value is not a char".to_string(),
                ))
            }
        });

        if y.is_none() || x.is_none() {
            return Err(ParsePointError::FailedParse(format!(
                "Not enough coordinates to be valid: {:?}",
                &coords
            )));
        }
        Ok(Location {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.x, self.y)
    }
}

impl cmp::PartialEq<String> for Location {
    fn eq(&self, other: &String) -> bool {
        let str_rep = format!("{}{}", self.x, self.y);
        str_rep.as_str() == other
    }
}

impl cmp::PartialEq<str> for Location {
    fn eq(&self, other: &str) -> bool {
        let str_rep = format!("{}{}", self.x, self.y);
        str_rep.as_str() == other
    }
}

impl cmp::PartialEq<Location> for Location {
    fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl cmp::PartialEq<&Location> for Location {
    fn eq(&self, other: &&Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// impl cmp::Ord for Location {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if self.x != other.x {
//             return self.x.cmp(&other.x);
//         }
//         self.y.cmp(&other.y)
//     }
// }

#[test]
fn test_location_from_str() {
    let loc = Location::from_str("A1").unwrap();
    assert_eq!(&loc.x, "A");
    assert_eq!(&loc.y, "1");
}
