use crate::schema::lectures;

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Lecture {
    pub id: i32,
    pub name: String,
    pub number_of_times: i32
}

#[derive(Serialize, Deserialize)]
pub struct LectureDTO {
    pub id: String,
    pub name: String,
    pub number_of_times: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "lectures"]
pub struct NewLecture {
    pub name: String,
    pub number_of_times: i32
}

#[derive(Serialize, Deserialize)]
pub struct IdsDTO {
    pub ids: Vec<String>
}