use crate::schema::staffs;

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

#[derive(Serialize, Deserialize)]
pub struct StaffDTO {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub faculty: String,
    pub lectures: Vec<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "staffs"]
pub struct NewStaff {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub soft_constraints: i32,
    pub faculty: i32,
    pub role: String
}