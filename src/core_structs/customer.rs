use rusqlite::ToSql;
use rusqlite::types::ToSqlOutput;
use crate::core_structs::car::Car;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub customer_id: i32,
    pub customer_name: String,
    pub phone_number: String,
    pub car: Car,
}

impl ToSql for Customer {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let json = serde_json::to_vec(self).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        Ok(ToSqlOutput::Owned(json.into()))
    }
}