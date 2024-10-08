use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};
use axum::{
    routing::{get, post},
    Router,
};
use registry::AppRegistry;
pub fn build_book_routers() -> Router<AppRegistry> {
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route(
            "/:book_id",
            get(show_book).put(update_book).delete(delete_book),
        );

    Router::new().nest("/books", book_routers)
}
