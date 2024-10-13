use crate::models::invoice_details::InvoiceDetails;
use crate::models::customer::Customer;
use crate::models::car::Car;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_number: i32,
    pub date: String,
    pub details: Vec<InvoiceDetails>,
    pub customer: Customer,
    pub car: Car
}

