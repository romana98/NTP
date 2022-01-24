use std::fmt;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
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