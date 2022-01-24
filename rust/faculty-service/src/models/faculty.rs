use crate::schema::faculties;

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Facultie {
    pub id: i32,
    pub name: String,
    pub hard_constraint_id: i32,
    pub schedule_id: Option<i32>
}
pub type Faculty = Facultie;

#[derive(Serialize, Deserialize)]
pub struct FacultyDTO {
    pub id: String,
    pub name: String,
    pub hard_constraint: String,
    pub schedule: String,
    pub shifts: Vec<String>,
    pub staff: Vec<String>,
    pub lectures: Vec<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "faculties"]
pub struct NewFaculty {
    pub name: String,
    pub hard_constraint_id: i32,
    pub schedule_id:  Option<i32>
}