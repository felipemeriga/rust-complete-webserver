#[cfg(test)]
mod tests {
    use crate::api::user_api::delete_user;
    use crate::api::user_api::{create_user, get_all_users, get_user};
    use crate::auth::api_key::ApiKeyData;
    use crate::auth::auth0::Auth0Data;
    use crate::configuration::config::Config;
    use crate::database::repository::MockRepository;
    use crate::models::app::AppData;
    use crate::models::user_model::{CreateUserResult, DeleteUserResult, User};
    use actix_web::web::Data;
    use actix_web::{test, App};
    use awc::http;
    use mockall::predicate;
    use mockall::predicate::*;
    use serde_json;
    use std::sync::Arc;

    const USER_ID: &str = "65f0bbf848c60e78920bfd4c";
    fn get_app_data() -> Data<AppData> {
        let mut mock = MockRepository::new();

        mock.expect_get_all_users().returning(|| {
            Ok(vec![User {
                id: None,
                name: "test".to_string(),
                title: "test".to_string(),
                location: "test".to_string(),
            }])
        });

        mock.expect_get_user()
            .with(predicate::eq(USER_ID.to_string()))
            .returning(|_| {
                Ok(Some(User {
                    id: None,
                    name: "test".to_string(),
                    title: "test".to_string(),
                    location: "test".to_string(),
                }))
            });

        mock.expect_create_user()
            .with(predicate::eq(User {
                id: None,
                name: "test".to_string(),
                title: "test".to_string(),
                location: "test".to_string(),
            }))
            .returning(|_| {
                Ok(CreateUserResult {
                    id: "test".to_string(),
                })
            });

        mock.expect_delete_user()
            .with(predicate::eq(USER_ID.to_string()))
            .returning(|_| Ok(DeleteUserResult { deleted_count: 1 }));

        let app_data = AppData {
            db: Arc::new(mock),
            config: Config {
                auth0_data: Auth0Data {
                    audience: "".to_string(),
                    domain: "".to_string(),
                },
                env: "test".to_string(),
                api_key_data: ApiKeyData {
                    api_key: "".to_string(),
                    enable_api_key: false,
                },
                mongo_uri: "".to_string(),
            },
        };
        Data::new(app_data)
    }

    #[actix_web::test]
    async fn test_get_all_users() {
        let mut app = test::init_service(
            App::new()
                .app_data(get_app_data().clone())
                .service(get_all_users),
        )
        .await;

        let req = test::TestRequest::with_uri("/users").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), 200);

        let body = test::read_body(resp).await;
        let users = serde_json::from_slice::<Vec<User>>(body.as_ref()).unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users.get(0).unwrap().name, "test")
    }

    #[actix_web::test]
    async fn test_get_user() {
        let mut app = test::init_service(
            App::new()
                .app_data(get_app_data().clone())
                .service(get_user),
        )
        .await;

        let req = test::TestRequest::with_uri(format!("/user/{}", USER_ID).as_str()).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), 200);

        let body = test::read_body(resp).await;
        let users = serde_json::from_slice::<User>(body.as_ref()).unwrap();
        assert_eq!(users.name, "test")
    }

    #[actix_web::test]
    async fn test_create_user() {
        let user_to_create = User {
            name: "test".to_string(),
            id: None,
            location: "test".to_string(),
            title: "test".to_string(),
        };
        let mut app = test::init_service(
            App::new()
                .app_data(get_app_data().clone())
                .service(create_user),
        )
        .await;

        let req = test::TestRequest::with_uri("/user")
            .method(http::Method::POST)
            .set_json(user_to_create)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), 200);
        //
        let body = test::read_body(resp).await;
        let create_user_result = serde_json::from_slice::<CreateUserResult>(body.as_ref()).unwrap();
        assert_eq!(create_user_result.id, "test")
    }

    #[actix_web::test]
    async fn test_delete_user() {
        let mut app = test::init_service(
            App::new()
                .app_data(get_app_data().clone())
                .service(delete_user),
        )
        .await;

        let req = test::TestRequest::with_uri(format!("/user/{}", USER_ID).as_str())
            .method(http::Method::DELETE)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), 200);
        //
        let body = test::read_body(resp).await;
        let delete_user_result = serde_json::from_slice::<String>(body.as_ref()).unwrap();
        assert_eq!(delete_user_result, "User successfully deleted!")
    }
}
