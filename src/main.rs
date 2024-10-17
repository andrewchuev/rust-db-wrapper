use bigdecimal::BigDecimal;
use sqlx::mysql::MySqlPool;
use sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let pool = MySqlPool::connect("mysql://homestead:secret@192.168.88.60/crlshop").await?;

    let row: (BigDecimal,) = sqlx::query_as("SELECT price FROM products WHERE id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await?;

    let price: BigDecimal = row.0;
    println!("Цена: {}", price);

    Ok(())
}
