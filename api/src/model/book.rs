use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json};
use kernel::model::book::{event::CreateBook, Book};
use registry::AppRegistry;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest { title, author, isbn, description } = value;

        Self { title, author, isbn, description }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book { id, title, author, isbn, description } = value;
        Self { id, title, author, isbn, description }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()       
    }
}

pub async fn register_book(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(AppError::from)
}

pub  async  fn show_book_list(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<BookResponse>>, AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect())
        .map(Json)
        .map_err(AppError::from)
}

pub async fn show_book(
    Path(book_id): Path<Uuid>,
    State(registry): State<AppRegistry>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc)=> Ok(Json(bc.into())),
            None => Err(anyhow::anyhow!("The specific book was not found")),
        })
        .map_err(AppError::from)
}