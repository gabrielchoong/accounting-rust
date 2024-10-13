use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceDetails {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
}

impl InvoiceDetails {
    
    pub fn new(description: String, quantity: i32, unit_price: f64) -> Self {

        InvoiceDetails {
            description,
            quantity,
            unit_price
        }
    }

    pub fn sum_of_item(&self) -> f64 {
        self.quantity as f64 * self.unit_price
    }
}
