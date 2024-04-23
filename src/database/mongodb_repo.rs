use async_trait::async_trait;
use futures_util::TryStreamExt;

use crate::database::error::RepositoryError;
use crate::database::repository::Repository;
use crate::models::user_model::{CreateUserResult, DeleteUserResult, UpdateUserResult, User};
use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::ClientOptions,
    Client, Collection,
};

#[derive(Debug, Clone)]
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init(uri: String) -> Self {
        let client_options = ClientOptions::parse(uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }
}

#[async_trait]
impl Repository for MongoRepo {
    async fn create_user(&self, new_user: User) -> Result<CreateUserResult, RepositoryError> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let result = self
            .col
            .insert_one(new_doc, None)
            .await
            .map_err(|err| RepositoryError::CreateUpdateUser(Box::from(err)))?;
        match result.inserted_id {
            Bson::ObjectId(object_id) => Ok(CreateUserResult {
                id: object_id.to_hex(),
            }),
            _ => Err(RepositoryError::GeneralError(
                "Error parsing id of created user".to_string(),
            )),
        }
    }

    async fn get_user(&self, id: String) -> Result<Option<User>, RepositoryError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.find_one(filter, None).await.unwrap();

        Ok(user_detail)
    }

    async fn update_user(&self, id: &str, user: User) -> Result<UpdateUserResult, RepositoryError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
                      "$set":
                    {
                        "id": user.id,
                        "name": user.name,
                        "location": user.location,
                        "title": user.title
                    },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .map_err(|err| RepositoryError::CreateUpdateUser(Box::from(err)))?;
        Ok(UpdateUserResult {
            matched_count: updated_doc.matched_count,
            modified_count: updated_doc.modified_count,
            upserted_id: "".to_string(),
        })
    }

    async fn delete_user(&self, id: &str) -> Result<DeleteUserResult, RepositoryError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let delete_result = self
            .col
            .delete_one(filter, None)
            .await
            .map_err(|err| RepositoryError::DeleteUser(Box::from(err)))?;
        Ok(DeleteUserResult {
            deleted_count: delete_result.deleted_count,
        })
    }

    async fn get_all_users(&self) -> Result<Vec<User>, RepositoryError> {
        let mut cursors = self.col.find(None, None).await.map_err(|_| {
            RepositoryError::GeneralError("Error getting list of users".to_string())
        })?;
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors.try_next().await.map_err(|_| {
            RepositoryError::GeneralError("Error mapping through cursor".to_string())
        })? {
            users.push(user)
        }
        Ok(users)
    }
}
