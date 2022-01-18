use crate::schema::lectures_faculties;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct LecturesFacultie {
    pub id: i32,
    pub faculty_id: i32,
    pub lecture_id: i32
}
pub type LecturesFaculty = LecturesFacultie;

#[derive(Serialize, Deserialize)]
pub struct LecturesFacultyDTO {
    pub id: i32,
    pub faculty_id: i32,
    pub lecture_id: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "lectures_faculties"]
pub struct NewLecturesFacultie {
    pub faculty_id: i32,
    pub lecture_id: i32
}
pub type NewLecturesFaculty = NewLecturesFacultie;