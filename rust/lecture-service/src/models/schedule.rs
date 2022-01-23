#[derive(Serialize, Deserialize)]
pub struct LectureSchedule {
    pub id: i32,
    pub name: String,
    pub number_of_times: i32
}