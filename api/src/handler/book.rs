use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::{id::BookId, model::book::event::DeleteBook};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthrorizedUser,
    model::book::{
        BookListQuery, BookResponse, CreateBookRequest, PagenatedookResponse, UpdateBookRequest,
        UpdateBookRequestWithIds,
    },
};

pub async fn register_book(
    user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    req.validate(&())?;
    registry
        .book_repository()
        .create(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    _user: AuthrorizedUser,
    State(registry): State<AppRegistry>,
    Query(query): Query<BookListQuery>,
) -> Result<Json<PagenatedookResponse>, AppError> {
    query.validate(&())?;

    registry
        .book_repository()
        .find_all(query.into())
        .await
        .map(PagenatedookResponse::from)
        .map(Json)
}

pub async fn show_book(
    _user: AuthrorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".into(),
            )),
        })
        .map_err(AppError::from)
}

pub async fn update_book(
    user: AuthrorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let event = UpdateBookRequestWithIds::new(book_id, user.id(), req);

    registry
        .book_repository()
        .update(event.into())
        .await
        .map(|_| StatusCode::OK)
}

pub async fn delete_book(
    user: AuthrorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let event = DeleteBook {
        book_id,
        requested_user: user.id(),
    };

    registry
        .book_repository()
        .delete(event)
        .await
        .map(|_| StatusCode::OK)
}
