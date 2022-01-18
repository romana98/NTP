use crate::schema::staffs_faculties;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct StaffsFacultie {
    pub id: i32,
    pub faculty_id: i32,
    pub staff_id: i32
}
pub type StaffFaculty = StaffsFacultie;

#[derive(Serialize, Deserialize)]
pub struct StaffFacultyDTO {
    pub id: i32,
    pub faculty_id: i32,
    pub staff_id: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "staffs_faculties"]
pub struct NewStaffsFacultie {
    pub faculty_id: i32,
    pub staff_id: i32
}
pub type NewStaffFaculty = NewStaffsFacultie;