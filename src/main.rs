use bigdecimal::BigDecimal;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};
use sqlx::Error;
use sqlx::FromRow;
use std::collections::HashMap;
use std::env;
use thiserror::Error;

#[derive(Debug, sqlx::FromRow)]
struct Product {
    id: u32,
    name: String,
    price: BigDecimal,
    description: Option<String>,
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

struct Repository {
    pool: MySqlPool,
}

impl Repository {
    pub fn new(pool: MySqlPool) -> Self {
        Repository { pool }
    }

    pub async fn fetch_all<T>(&self, table_name: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<T>, FetchError>
    where
        T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
    {
        let mut query = format!("SELECT * FROM {}", table_name);
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let items: Vec<T> = sqlx::query_as::<_, T>(&query)
            .fetch_all(&self.pool)
            .await?;

        if items.is_empty() {
            return Err(FetchError::NoRecordsFound(table_name.to_string()));
        }

        Ok(items)
    }

    pub async fn fetch_one<T>(&self, table_name: &str, id: u32) -> Result<T, FetchError>
    where
        T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
    {
        let query = format!("SELECT * FROM {} WHERE id = ?", table_name);
        let item: Option<T> = sqlx::query_as::<_, T>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match item {
            Some(record) => Ok(record),
            None => Err(FetchError::NoRecordFound(id)),
        }
    }

    pub async fn insert_record(&self, table_name: &str, fields: HashMap<&str, &str>) -> Result<u64, FetchError> {
        let columns: Vec<&str> = fields.keys().cloned().collect();
        let placeholders: Vec<String> = (0..fields.len()).map(|_| "?".to_string()).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let mut query_builder = sqlx::query(&query);
        for value in fields.values() {
            query_builder = query_builder.bind(*value);
        }

        let result: MySqlQueryResult = query_builder.execute(&self.pool).await?;
        Ok(result.last_insert_id())
    }

    pub async fn update_record(&self, table_name: &str, id: u32, fields: HashMap<&str, &str>) -> Result<u64, FetchError> {
        let set_clauses: Vec<String> = fields.keys().map(|key| format!("{} = ?", key)).collect();
        let query = format!(
            "UPDATE {} SET {} WHERE id = ?",
            table_name,
            set_clauses.join(", ")
        );

        let mut query_builder = sqlx::query(&query);
        for value in fields.values() {
            query_builder = query_builder.bind(*value);
        }
        query_builder = query_builder.bind(id);

        let result: MySqlQueryResult = query_builder.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_record(&self, table_name: &str, id: u32) -> Result<u64, FetchError> {
        let query = format!("DELETE FROM {} WHERE id = ?", table_name);
        let result: MySqlQueryResult = sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[tokio::main]
async fn main() -> Result<(), FetchError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    let repo = Repository::new(pool);

    // Fetch all products with pagination
    match repo.fetch_all::<Product>("products", Some(10), Some(0)).await {
        Ok(products) => {
            for product in products {
                println!("{:?}", product);
            }
        }
        Err(e) => println!("Error fetching products: {}", e),
    }

    // Fetch a single product by ID
    match repo.fetch_one::<Product>("products", 1).await {
        Ok(product) => println!("Fetched product: {:?}", product),
        Err(e) => println!("Error fetching product by ID: {}", e),
    }

    // Insert a new product
    let mut new_product_fields = HashMap::new();
    new_product_fields.insert("name", "New Product");
    new_product_fields.insert("price", "99.99");
    new_product_fields.insert("description", "A newly added product");
    new_product_fields.insert("category_id", "1");

    match repo.insert_record("products", new_product_fields).await {
        Ok(id) => println!("Inserted new product with ID: {}", id),
        Err(e) => println!("Error inserting product: {}", e),
    }

    // Update a product
    let mut update_fields = HashMap::new();
    update_fields.insert("name", "Updated Product Name");
    update_fields.insert("price", "199.99");

    match repo.update_record("products", 1, update_fields).await {
        Ok(rows) => println!("Updated {} row(s)", rows),
        Err(e) => println!("Error updating product: {}", e),
    }

    // Delete a product
    match repo.delete_record("products", 1).await {
        Ok(rows) => println!("Deleted {} row(s)", rows),
        Err(e) => println!("Error deleting product: {}", e),
    }

    Ok(())
}
