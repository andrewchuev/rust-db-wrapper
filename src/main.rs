mod repository;

mod queries;
mod models;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use sqlx::Error;
use sqlx::FromRow;

use std::env;

use thiserror::Error;

use crate::queries::queries::{get_post, get_posts};

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


#[tokio::main]
async fn main() -> Result<(), FetchError> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    let repo = Repository::new(pool);

    get_posts(&repo).await;
    get_post(&repo, 26458).await;


    // Fetch all products with pagination
    /*match repo.fetch_all::<Product>("products", Some(10), Some(0)).await {
        Ok(products) => {
            for product in products {
                println!("{:?}", product);
            }
        }
        Err(e) => println!("Error fetching products: {}", e),
    }*/

    // Fetch a single product by ID
    /*match repo.fetch_one::<Product>("products", 110).await {
        Ok(product) => println!("Fetched product: {:?}", product),
        Err(e) => println!("Error fetching product by ID: {}", e),
    }*/

    // Insert a new product
    /*let mut new_product_fields = HashMap::new();
    new_product_fields.insert("name", "New Product");
    new_product_fields.insert("price", "99.99");
    new_product_fields.insert("description", "A newly added product");
    new_product_fields.insert("category_id", "1");

    match repo.insert_record("products", new_product_fields).await {
        Ok(id) => println!("Inserted new product with ID: {}", id),
        Err(e) => println!("Error inserting product: {}", e),
    }*/

    // Update a product
    /*let mut update_fields = HashMap::new();
    update_fields.insert("name", "Updated Product Name");
    update_fields.insert("price", "199.99");

    match repo.update_record("products", 110, update_fields).await {
        Ok(rows) => println!("Updated {} row(s)", rows),
        Err(e) => println!("Error updating product: {}", e),
    }*/

    // Delete a product
    /*match repo.delete_record("products", 110).await {
        Ok(rows) => println!("Deleted {} row(s)", rows),
        Err(e) => println!("Error deleting product: {}", e),
    }*/

    Ok(())
}
