use crate::schema::shifts_faculties;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct ShiftsFacultie {
    pub id: i32,
    pub faculty_id: i32,
    pub shift_id: i32
}
pub type ShiftsFaculty = ShiftsFacultie;

#[derive(Serialize, Deserialize)]
pub struct ShiftsFacultyDTO {
    pub id: i32,
    pub faculty_id: i32,
    pub shift_id: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "shifts_faculties"]
pub struct NewShiftsFacultie {
    pub faculty_id: i32,
    pub shift_id: i32
}
pub type NewShiftsFaculty = NewShiftsFacultie;