use axum::{Router,routing};

use crate::api::handlers::*;

pub fn app_routes() -> Router {
    Router::new()
        .route("/users", routing::get(get_users))
        .route("/users", routing::post(create_user))

        .route("/posts", routing::post(create_post))
        .route("/posts", routing::get(read_post_all))
        .route("/posts/{id}", routing::get(read_post))
        .route("/posts/{id}", routing::put(update_post))
        .route("/posts/{id}", routing::delete(delete_post))
}
