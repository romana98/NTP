#[derive(Serialize, Deserialize)]
pub struct ShiftDTO {
    pub id: String,
    pub start: String,
    pub end: String,
    pub day: String
}