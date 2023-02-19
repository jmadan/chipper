use axum::{
    routing::{get},
    Router
};

use super::users;

pub fn router() -> Router {
    let user_routes = Router::new()
        .route("/", get(users::handler::list));

    let api_routes = Router::new()
        .nest("/users", user_routes);

    Router::new()
        .nest("/", api_routes)
}

async fn get_users() -> &'static str {
    "Hello, Chipper User!"
}
