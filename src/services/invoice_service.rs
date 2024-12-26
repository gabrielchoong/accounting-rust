use crate::models::invoice::Invoice;
use crate::models::invoice_details::InvoiceDetails;
use crate::models::customer::Customer;
use crate::models::car::Car;

#[derive(Debug)]
pub struct InvoiceService;

impl InvoiceService {

    fn create_details() -> Vec<InvoiceDetails> {

        // suppose this returns a Vector of InvoiceDetails

        let details_1 = InvoiceDetails::new(
            "Oil Change".to_string(),
            1,
            150.00,
        );

        let details_2 = InvoiceDetails::new(
            "Brake Service".to_string(),
            1,
            80.00,
        );

        let v = vec![details_1, details_2];
        v
    }

    pub fn create_invoice() -> Invoice {

        let details = InvoiceService::create_details();

        let customer = Customer::get_customer();

        let car = Car {
            id: 1,
            model: "Kia Sentra".to_string(),
            registration_number: "ABC1234".to_string()
        };

        Invoice {
            invoice_number: 1,
            date: "2024-10-13".to_string(),
            details: details,
            customer: customer,
            car: car
        }
    }
}
