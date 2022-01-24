#[derive(Serialize, Deserialize)]
pub struct StaffDTO {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub faculty: String,
    pub lectures: Vec<String>
}