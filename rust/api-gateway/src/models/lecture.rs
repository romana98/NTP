#[derive(Serialize, Deserialize)]
pub struct LectureDTO {
    pub id: String,
    pub name: String,
    pub number_of_times: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct IdsDTO {
    pub ids: Vec<String>
}