use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SoftConstrainstDTO {
    pub id: String,
    pub prefers: HashMap<String, i32>
}
