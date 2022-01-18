use crate::schema::shifts;

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Shift {
    pub id: i32,
    pub start_shift: String,
    pub end_shift: String,
    pub day: String
}

#[derive(Serialize, Deserialize)]
pub struct ShiftDTO {
    pub id: String,
    pub start: String,
    pub end: String,
    pub day: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "shifts"]
pub struct NewShift {
    pub start_shift: String,
    pub end_shift: String,
    pub day: String
}
