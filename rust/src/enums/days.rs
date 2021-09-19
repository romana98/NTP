use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

impl Display for Days {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Days::Monday => {
                write!(f, "monday")
            }
            Days::Tuesday => {
                write!(f, "tuesday")
            }
            Days::Wednesday => {
                write!(f, "tuesday")
            }
            Days::Thursday => {
                write!(f, "tuesday")
            }
            Days::Friday => {
                write!(f, "tuesday")
            }
        }
    }
}