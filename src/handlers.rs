use axum::{extract, http};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    pub fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateQuote {
    book: Option<String>,
    quote: Option<String>,
}

pub async fn health() -> http::StatusCode {
    println!("Health check");
    http::StatusCode::OK
}

pub async fn create_quote(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.inserted_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>(
        r#"
        SELECT id, book, quote, inserted_at, updated_at
        FROM quotes
        "#,
    )
    .fetch_all(&pool)
    .await;
    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quote(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<UpdateQuote>,
) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET book = $1, quote = $2
        WHERE id = $3
        RETURNING id, book, quote, inserted_at, updated_at
        "#,
    )
    .bind(&payload.book)
    .bind(&payload.quote)
    .bind(&id)
    .fetch_optional(&pool)
    .await
    // Err variant
    .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?
    // None variant
    .ok_or(http::StatusCode::NOT_FOUND)?;

    let new_quote = Quote::from_row(&res).map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((http::StatusCode::OK, axum::Json(new_quote)))
}

pub async fn delete_quote(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<PgPool>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = sqlx::query(
        r#"
        DELETE FROM quotes
        WHERE id = $1
        "#,
    )
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?
    .rows_affected();

    match res {
        0 => Err(http::StatusCode::NOT_FOUND),
        _ => Ok(http::StatusCode::NO_CONTENT),
    }
}
