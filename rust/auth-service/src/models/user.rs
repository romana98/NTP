use crate::schema::{staffs, admins};


#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub soft_constraints: i32,
    pub faculty: i32,
    pub role: String
}

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub admin_email: String,
    pub password: String,
    pub role: String
}