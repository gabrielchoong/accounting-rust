use crate::core_structs::customer::Customer;
use crate::core_structs::invoice_item::{InvoiceItem, InvoiceItems};

use crate::utils::db::{delete_data, get_data_with_id, insert_data, update_data};
use chrono::prelude::*;

#[derive(Debug)]
pub struct Invoice {
    // pub invoice_number: i32,
    pub date: String,
    pub details: Vec<InvoiceItem>,
    pub customer: Customer,
}

impl Invoice {
    pub fn new(
        // invoice_number: i32,
        date: String,
        details: Vec<InvoiceItem>,
        customer: Customer,
    ) -> Self {
        Invoice {
            // invoice_number,
            date,
            details,
            customer,
        }
    }

    fn get_current_date() -> String {
        let local: DateTime<Local> = Local::now();

        local.format("%Y-%m-%d").to_string()
    }

    pub fn create_invoice(
        &self,
        details: InvoiceItems,
        customer: Customer,
    ) -> Result<(), rusqlite::Error> {
        let date: String = Self::get_current_date();

        insert_data(date, details, customer, None)?;

        Ok(())
    }

    pub fn get_invoice_with_id(&self, invoice_number: i32) -> Result<Invoice, rusqlite::Error> {
        let invoice = get_data_with_id(invoice_number, None);

        invoice
    }

    pub fn update_invoice(&self, invoice_number: i32) -> Result<(), rusqlite::Error> {
        update_data(invoice_number, None, None, None)?;

        Ok(())
    }

    pub fn delete_invoice(&self, invoice_number: i32) -> Result<(), rusqlite::Error> {
        delete_data(invoice_number, None)?;

        Ok(())
    }

    // optional

    #[warn(dead_code)]
    pub fn export_invoice(&self, _keyword: &str) {}
}
