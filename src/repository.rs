use crate::{FetchError, Repository};
use sqlx::mysql::MySqlQueryResult;
use sqlx::{FromRow, MySqlPool};
use std::collections::HashMap;

impl Repository {
    pub fn new(pool: MySqlPool) -> Self {
        Repository { pool }
    }

    pub async fn fetch_all<T>(&self, table_name: &str, limit: Option<u32>, offset: Option<u32>, condition: Option<&str>) -> Result<Vec<T>, FetchError>
    where
        T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Unpin + Send,
    {
        let mut query = format!("SELECT * FROM {}", table_name);

        if let Some(condition) = condition {
            query.push_str(&format!(" WHERE {}", condition));
        }

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        println!("SQL: {}", query);

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
