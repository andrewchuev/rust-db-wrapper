use bigdecimal::BigDecimal;
use sqlx::mysql::MySqlPool;
use sqlx::Error;
use sqlx::FromRow;
use thiserror::Error;
use dotenv::dotenv;
use std::env;

#[derive(Debug, sqlx::FromRow)]
struct Product {
    id: u32,
    name: String,
    price: BigDecimal,
    description: Option<String>, // Добавлено поле описания, если оно есть в таблице
}

#[derive(Error, Debug)]
enum FetchError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] Error),
    #[error("No records found in table {0}")]
    NoRecordsFound(String),
    #[error("No record found with id {0}")]
    NoRecordFound(u32),
}

async fn fetch_all<T>(pool: &MySqlPool, table_name: &str) -> Result<Vec<T>, FetchError>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
{
    let query = format!("SELECT * FROM {}", table_name);
    let items: Vec<T> = sqlx::query_as::<_, T>(&query)
        .fetch_all(pool)
        .await?;

    if items.is_empty() {
        return Err(FetchError::NoRecordsFound(table_name.to_string()));
    }

    Ok(items)
}

async fn fetch_one<T>(pool: &MySqlPool, table_name: &str, id: u32) -> Result<T, FetchError>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
{
    let query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    let item: Option<T> = sqlx::query_as::<_, T>(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?;

    match item {
        Some(record) => Ok(record),
        None => Err(FetchError::NoRecordFound(id)),
    }
}

#[tokio::main]
async fn main() -> Result<(), FetchError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    match fetch_all::<Product>(&pool, "products").await {
        Ok(products) => {
            for product in products {
                println!("{:?}", product);
            }
        }
        Err(e) => println!("Error fetching products: {}", e),
    }

    match fetch_one::<Product>(&pool, "products", 1).await {
        Ok(product) => println!("Fetched product: {:?}", product),
        Err(e) => println!("Error fetching product by ID: {}", e),
    }

    Ok(())
}
