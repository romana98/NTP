use std::fmt;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Staff
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Staff => write!(f, "ROLE_STAFF"),
            Role::Admin => write!(f, "ROLE_ADMINISTRATOR"),
        }
    }
}