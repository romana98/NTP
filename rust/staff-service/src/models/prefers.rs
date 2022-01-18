use crate::schema::prefers;

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Prefer {
    pub id: i32,
    pub soft_c_id: i32,
    pub day: String,
    pub num: i32
}

#[derive(Serialize, Deserialize)]
pub struct PrefersDTO {
    pub id: i32,
    pub soft_c_id: i32,
    pub day: String,
    pub num: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "prefers"]
pub struct NewPrefers {
    pub soft_c_id: i32,
    pub day: String,
    pub num: i32
}
