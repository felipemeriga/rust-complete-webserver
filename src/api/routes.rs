use crate::api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use crate::auth::auth_middleware::AuthMiddleware;
use actix_web::web::scope;
use actix_web::Scope;

pub fn routes(auth_middleware: AuthMiddleware) -> Scope {
    scope("/api")
        .service(
            scope("/admin")
                .wrap(auth_middleware)
                .service(create_user)
                .service(update_user)
                .service(delete_user),
        )
        .service(scope("").service(get_user).service(get_all_users))
}
