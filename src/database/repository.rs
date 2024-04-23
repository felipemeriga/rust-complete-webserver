use crate::database::error::RepositoryError;
use crate::models::user_model::{CreateUserResult, DeleteUserResult, UpdateUserResult, User};
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use std::fmt::Debug;

#[automock]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_user(&self, new_user: User) -> Result<CreateUserResult, RepositoryError>;
    async fn get_user(&self, id: String) -> Result<Option<User>, RepositoryError>;
    async fn update_user(&self, id: &str, user: User) -> Result<UpdateUserResult, RepositoryError>;
    async fn delete_user(&self, id: &str) -> Result<DeleteUserResult, RepositoryError>;
    async fn get_all_users(&self) -> Result<Vec<User>, RepositoryError>;
}

impl Debug for dyn Repository {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{}}")
    }
}

// We can use this struct as an implementation of Repository trait, for mocking results in Unit tests
// however, we are using Mockall package
pub struct MockDatabase {
    pub test_user: User,
    pub should_error: bool,
    pub create_user_result: CreateUserResult,
    pub update_user_result: UpdateUserResult,
    pub delete_user_result: DeleteUserResult,
}

impl MockDatabase {
    async fn return_result<T>(&self, result: T) -> Result<T, RepositoryError> {
        if self.should_error {
            return Err(RepositoryError::GeneralError("test error".to_string()));
        }
        Ok(result)
    }
}

#[async_trait]
impl Repository for MockDatabase {
    async fn create_user(&self, _: User) -> Result<CreateUserResult, RepositoryError> {
        self.return_result(self.create_user_result.clone()).await
    }

    async fn get_user(&self, _: String) -> Result<Option<User>, RepositoryError> {
        self.return_result(Some(self.test_user.clone())).await
    }

    async fn update_user(&self, _: &str, _: User) -> Result<UpdateUserResult, RepositoryError> {
        self.return_result(self.update_user_result.clone()).await
    }

    async fn delete_user(&self, _: &str) -> Result<DeleteUserResult, RepositoryError> {
        self.return_result(self.delete_user_result.clone()).await
    }

    async fn get_all_users(&self) -> Result<Vec<User>, RepositoryError> {
        self.return_result(vec![self.test_user.clone()]).await
    }
}
