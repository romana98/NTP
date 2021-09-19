use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum Role {
    Admin,
    Staff,
    Both
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Role::Admin => {
                write!(f, "ROLE_ADMINISTRATOR")
            }
            Role::Staff => {
                write!(f, "ROLE_STAFF")
            }
            Role::Both => {
                write!(f, "ROLE_BOTH")
            }
        }
    }
}