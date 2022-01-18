use crate::schema::soft_constraints;
use std::collections::HashMap;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct SoftConstraint {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct SoftConstrainstDTO {
    pub id: String,
    pub prefers: HashMap<String, i32>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "soft_constraints"]
pub struct NewSoftConstraints {
    pub id: Option<i32>
}