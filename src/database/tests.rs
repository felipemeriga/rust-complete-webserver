#[cfg(test)]
mod tests {
    use crate::database::mongodb_repo::MongoRepo;
    use crate::database::repository::Repository;
    use crate::models::user_model::User;
    use testcontainers::clients::Cli;
    use testcontainers::GenericImage;

    #[tokio::test]
    async fn test_user_collection() {
        let docker = Cli::default();
        let image = GenericImage::new("mongo", "latest");
        let container = docker.run(image);
        let mongo_address = format!(
            "mongodb://localhost:{}/",
            container.get_host_port_ipv4(27017)
        );

        let mongo_repo = MongoRepo::init(mongo_address).await;

        let create_result = mongo_repo
            .create_user(User {
                id: None,
                name: "test".to_string(),
                location: "test".to_string(),
                title: "test".to_string(),
            })
            .await;
        assert!(!create_result.is_err());
        let user_id = create_result.unwrap().id;
        assert!(!user_id.is_empty());

        let get_user_result = mongo_repo.get_user(user_id.clone()).await;
        assert!(!get_user_result.is_err());
        let user = get_user_result.unwrap().unwrap();
        assert_eq!(user.name, "test");

        let update_user_result = mongo_repo
            .update_user(
                &user_id.clone(),
                User {
                    id: None,
                    name: "updated".to_string(),
                    location: "updated".to_string(),
                    title: "updated".to_string(),
                },
            )
            .await;
        assert!(!update_user_result.is_err());
        assert_eq!(update_user_result.unwrap().modified_count, 1);

        let delete_user_result = mongo_repo.delete_user(&user_id.clone()).await;
        assert!(!delete_user_result.is_err())
    }
}
