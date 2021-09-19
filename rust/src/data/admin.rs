use mongodb::bson;

#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub role: String,
}