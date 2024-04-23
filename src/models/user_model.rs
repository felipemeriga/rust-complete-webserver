use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResult {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserResult {
    pub matched_count: u64,
    pub modified_count: u64,
    pub upserted_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteUserResult {
    pub deleted_count: u64,
}

// TODO - This is the correct way of
#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    // rest of fields
}
