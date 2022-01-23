use std::fmt;
use strum_macros::EnumIter; 

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Day {
    pub fn from_str(role: &str) -> Day {
        match role {
            "monday" => Day::Monday,
            "tuesday" => Day::Tuesday,
            "wednesday" => Day::Wednesday,
            "thursday" => Day::Thursday,
            _ => Day::Friday,
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Monday => write!(f, "monday"),
            Day::Tuesday => write!(f, "tuesday"),
            Day::Wednesday => write!(f, "wednesday"),
            Day::Thursday => write!(f, "thursday"),
            Day::Friday => write!(f, "friday")
        }
    }
}