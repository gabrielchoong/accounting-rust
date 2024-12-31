#[derive(Debug)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
}

// TODO: Implement enum for handling errors replacing strings in prod

impl InvoiceItem {
    pub fn new(description: String, quantity: i32, unit_price: f64) -> Result<Self, String> {

        if description.trim().is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if quantity <= 0 {
            return Err("Quantity must be greater than 0".to_string());
        }
        if unit_price <= 0.0 {
            return Err("Unit Price must be greater than 0".to_string());
        }

        Ok(InvoiceItem {
            description,
            quantity,
            unit_price,
        })
    }

    pub fn get_price(&self) -> f64 {

        self.quantity as f64 * (self.unit_price * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_item_creation_success() {
        let item = InvoiceItem::new("Item label".to_string(), 2, 9.99);
        assert!(item.is_ok());
    }

    #[test]
    fn test_invoice_item_creation_negative_quantity() {
        let item = InvoiceItem::new("Item label".to_string(), -2, 9.99);
        assert_eq!(item.unwrap_err(), "Quantity must be greater than 0".to_string());
    }

    #[test]
    fn test_invoice_item_creation_zero_quantity() {
        let item = InvoiceItem::new("Item label".to_string(), 0, 9.99);
        assert_eq!(item.unwrap_err(), "Quantity must be greater than 0".to_string());
    }

    #[test]
    fn test_invoice_item_creation_negative_price() {
        let item = InvoiceItem::new("Item label".to_string(), 2, -9.99);
        assert_eq!(item.unwrap_err(), "Unit Price must be greater than 0".to_string());
    }

    #[test]
    fn test_invoice_item_creation_zero_price() {
        let item = InvoiceItem::new("Item label".to_string(), 2, 0.00);
        assert_eq!(item.unwrap_err(), "Unit Price must be greater than 0".to_string());
    }

    #[test]
    fn test_invoice_item_creation_no_description() {
        let item = InvoiceItem::new("".to_string(), 2, 9.99);
        assert_eq!(item.unwrap_err(), "Description cannot be empty".to_string());
    }

    #[test]
    fn test_invoice_item_get_price_success() {
        let item = InvoiceItem::new("Item label".to_string(), 2, 9.99);
        assert_eq!(item.unwrap().get_price(), 19.98);
    }
}
