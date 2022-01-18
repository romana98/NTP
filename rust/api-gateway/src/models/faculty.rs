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

#[derive(Serialize, Deserialize)]
pub struct StaffFacultyDTO {
    pub id: i32,
    pub faculty_id: i32,
    pub staff_id: i32
}
