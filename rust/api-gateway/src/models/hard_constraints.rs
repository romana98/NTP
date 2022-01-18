#[derive(Serialize, Deserialize)]
pub struct HardConstraintDTO {
    pub id: String,
    pub daily_max: i32,
    pub weekly_max: i32,
    pub weekly_min: i32,
    pub max_per_shift: i32 
}