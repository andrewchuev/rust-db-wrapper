use bigdecimal::BigDecimal;
#[derive(Debug, sqlx::FromRow)]
pub struct Product {
    id: u32,
    name: String,
    price: BigDecimal,
    description: Option<String>,
}