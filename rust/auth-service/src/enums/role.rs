use std::fmt;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Staff
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "ROLE_ADMINISTRATOR" => Role::Admin,
            _ => Role::Staff,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Staff => write!(f, "ROLE_STAFF"),
            Role::Admin => write!(f, "ROLE_ADMINISTRATOR"),
        }
    }
}