use crate::core_structs::customer::Customer;
use crate::core_structs::invoice::Invoice;
use crate::core_structs::invoice_item::InvoiceItems;
use crate::core_structs::car::Car;
use rusqlite::{params, Connection, Result};

const DB_PATH: &str = "src/data/invoice.db";

pub fn init_database(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS invoice (\
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        date TEXT NOT NULL, \
        details BLOB NOT NULL, \
        customer BLOB NOT NULL \
        )",
        (),
    )?;

    Ok(Connection::open(DB_PATH)?)
}

pub fn insert_data(
    date: String,
    details: InvoiceItems,
    customer: Customer,
    conn: Option<&Connection>,
) -> Result<()> {
    let conn = match conn {
        Some(conn) => conn,
        None => &init_database(DB_PATH)?,
    };

    conn.execute(
        "INSERT INTO invoice (date, details, customer) \
      VALUES (?1, ?2, ?3)",
        params![date, details, customer],
    )?;

    println!("Data inserted successfully!");

    Ok(())
}

pub fn get_data_with_id(id: i32, conn: Option<&Connection>) -> Result<Invoice> {
    let conn = match conn {
        Some(conn) => conn,
        None => &init_database(DB_PATH)?,
    };

    let mut stmt = conn.prepare(
        "SELECT id, date, details, customer \
        FROM invoice \
        WHERE id = ?1",
    )?;

    let invoice = stmt.query_row(params![id], |row| {
        let details_blob: Vec<u8> = row.get(2)?;
        let customer_blob: Vec<u8> = row.get(3)?;

        let details: InvoiceItems = serde_json::from_slice(&details_blob).expect("REASON");
        let customer: Customer = serde_json::from_slice(&customer_blob).expect("REASON");

        Ok(Invoice {
            // invoice_number: row.get(0)?,
            date: row.get(1)?,
            details: details.into_vec(),
            customer,
        })
    })?;

    Ok(invoice)
}

pub fn get_all_data(conn: Option<&Connection>) -> Result<Vec<Invoice>> {
    let conn = match conn {
        Some(conn) => conn,
        None => &init_database(DB_PATH)?,
    };

    let mut stmt = conn.prepare(
        "SELECT id, date, details, customer \
        FROM invoice \
        ORDER BY date DESC",
    )?;

    let invoices_iter = stmt.query_map([], |row| {
        let details_blob: Vec<u8> = row.get(2)?;
        let customer_blob: Vec<u8> = row.get(3)?;

        let details: InvoiceItems = serde_json::from_slice(&details_blob).expect("REASON");
        let customer: Customer = serde_json::from_slice(&customer_blob).expect("REASON");

        Ok(Invoice {
            // invoice_number: row.get(0)?,
            date: row.get(1)?,
            details: details.into_vec(),
            customer,
        })
    })?;

    let invoices: Vec<Invoice> = invoices_iter.filter_map(Result::ok).collect();

    Ok(invoices)
}

pub fn update_data(
    id: i32,
    new_details: Option<&InvoiceItems>,
    new_customer: Option<&Customer>,
    conn: Option<&Connection>,
) -> Result<()> {
    let conn = match conn {
        Some(conn) => conn,
        None => &init_database(DB_PATH)?,
    };

    let mut query = String::from("UPDATE invoice SET ");
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut owned_blobs: Vec<Vec<u8>> = Vec::new();

    if let Some(details) = new_details {
        query.push_str("details = ?,");
        let details_blob = serde_json::to_vec(details).unwrap();
        owned_blobs.push(details_blob);
    }

    if let Some(customer) = new_customer {
        query.push_str("customer = ?,");
        let customer_blob = serde_json::to_vec(customer).unwrap();
        owned_blobs.push(customer_blob);
    }

    if let Some(details) = new_details {
        params.push(&owned_blobs[owned_blobs.len() - 2] as &dyn rusqlite::ToSql);
    }

    if let Some(customer) = new_customer {
        params.push(&owned_blobs[owned_blobs.len() - 1] as &dyn rusqlite::ToSql);
    }

    query.pop();

    query.push_str(" WHERE id = ?");

    params.push(&id as &(dyn rusqlite::ToSql));

    conn.execute(&query, params.as_slice())?;

    Ok(())
}

pub fn delete_data(id: i32, conn: Option<&Connection>) -> Result<()> {
    let conn = match conn {
        Some(conn) => conn,
        None => &init_database(DB_PATH)?,
    };

    conn.execute(
        "DELETE FROM invoice \
        WHERE id = ?1",
        params![id],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INVALID_DB_PATH: &str = "/root/not_exist.db";
    const TEMP_DB_PATH: &str = "test.db";

    #[test]
    fn init_database_success() {
        let conn = init_database(DB_PATH).unwrap();

        let table_exists: bool = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='invoice'")
            .unwrap()
            .exists([])
            .unwrap();

        assert!(table_exists, "The invoice table was not created!");
    }

    #[test]
    fn init_database_failure() {
        let _ = std::fs::remove_file(INVALID_DB_PATH);

        let conn = init_database(INVALID_DB_PATH);

        assert!(conn.is_err(), "Expected database connection to fail!");

        if let Err(e) = conn {
            assert!(
                e.to_string().contains("unable to open database"),
                "Unexpected error {}",
                e
            );
        }
    }

    #[test]
    fn init_database_is_idempotent() {
        assert!(
            init_database(DB_PATH).is_ok(),
            "init_database failed on first call!"
        );

        assert!(
            init_database(DB_PATH).is_ok(),
            "init_database failed on the second call!"
        );
    }

    #[test]
    fn insert_data_success() {
        let date: String = chrono::Utc::now().to_rfc3339();
        let details: InvoiceItems = InvoiceItems(vec![]);
        let customer: Customer = Customer {
            customer_id: 0,
            customer_name: "".to_string(),
            phone_number: "".to_string(),
            car: Car { car_model: "".to_string(), car_reg_number: "".to_string() },
        };
        let conn = init_database(TEMP_DB_PATH).unwrap();

        assert!(insert_data(date, details, customer, Some(&conn)).is_ok(), "Error inserting into database!");
    }

    #[test]
    fn get_data_with_id_success() {
        let date: String = chrono::Utc::now().to_rfc3339();
        let details: InvoiceItems = InvoiceItems(vec![]);
        let customer: Customer = Customer {
            customer_id: 0,
            customer_name: "".to_string(),
            phone_number: "".to_string(),
            car: Car { car_model: "".to_string(), car_reg_number: "".to_string() },
        };
        let conn = init_database(TEMP_DB_PATH).unwrap();

        insert_data(date, details, customer, Some(&conn)).unwrap();

        let result = get_data_with_id(1, Some(&conn));
        assert!(result.is_ok(), "Error reading the database: {:?}", result.err());
    }

    #[test]
    fn get_all_data_success() {
        let conn = init_database(TEMP_DB_PATH).unwrap();

        assert!(get_all_data(Some(&conn)).is_ok(), "Error reading the database!");
    }

    #[test]
    fn update_data_with_customer_success() {
        let details: InvoiceItems = InvoiceItems(vec![]);
        let customer: Customer = Customer {
            customer_id: 1,
            customer_name: "John Doe".to_string(),
            phone_number: "123456789".to_string(),
            car: Car { car_model: "Toyota".to_string(), car_reg_number: "XYZ123".to_string() },
        };

        assert!(update_data(2, Some(&details), Some(&customer), None).is_ok(), "Error updating into database!");
    }

    #[test]
    fn delete_data_with_customer_success() {
        assert!(delete_data(1, None).is_ok(), "Error deleting into database!");
    }
}
