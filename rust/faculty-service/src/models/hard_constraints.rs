use crate::schema::hard_constraints;

#[derive(Identifiable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct HardConstraint {
    pub id: i32,
    pub daily_max: i32,
    pub weekly_max: i32,
    pub weekly_min: i32,
    pub max_per_shift: i32
}

#[derive(Serialize, Deserialize)]
pub struct HardConstraintDTO {
    pub id: String,
    pub daily_max: i32,
    pub weekly_max: i32,
    pub weekly_min: i32,
    pub max_per_shift: i32 
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "hard_constraints"]
pub struct NewHardConstraint {
    pub daily_max: i32,
    pub weekly_max: i32,
    pub weekly_min: i32,
    pub max_per_shift: i32 
}