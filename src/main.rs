use bigdecimal::BigDecimal;
use sqlx::mysql::MySqlPool;
use sqlx::Error;
use sqlx::FromRow;

#[derive(Debug, sqlx::FromRow)]
struct Product {
    id: u64,
    name: String,
    price: BigDecimal,
    description: Option<String>, // Добавлено поле описания, если оно есть в таблице
}

async fn fetch_all<T>(pool: &MySqlPool, table_name: &str) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
{
    let query = format!("SELECT * FROM {}", table_name);
    let items: Vec<T> = sqlx::query_as::<_, T>(&query)
        .fetch_all(pool)
        .await?;
    Ok(items)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = MySqlPool::connect("mysql://homestead:secret@192.168.88.60/crlshop").await?;

    let products: Vec<Product> = fetch_all::<Product>(&pool, "products").await?;

    for product in products {
        println!("{:?}", product);
    }

    Ok(())
}