use async_trait::async_trait;
use derive_new::new;
use kernel::{
    id::BookId,
    model::book::{event::CreateBook, Book},
    repository::book::BookRepository,
};
use shared::error::{AppError, AppResult};
use sqlx;

use crate::database::{book::BookRow, ConnectionPool};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO
                    books (title, author, isbn, description)
                VALUES
                    ($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }

    async fn find_all(&self) -> AppResult<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM
                    books
                ORDER BY
                    created_at DESC 
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM
                    books
                WHERE
                    book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    #[ignore]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));

        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Descrption".into(),
        };

        repo.create(book).await?;

        let all_books = repo.find_all().await?;

        assert_eq!(all_books.len(), 1);

        let book_id = all_books[0].id;

        let created_book = repo.find_by_id(book_id).await?;
        assert!(created_book.is_some());

        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = created_book.unwrap();

        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Descrption");

        Ok(())
    }
}
