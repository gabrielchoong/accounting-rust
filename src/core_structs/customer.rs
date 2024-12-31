use crate::core_structs::car::Car;

#[derive(Debug)]
pub struct Customer {
    pub customer_id: i32,
    pub customer_name: String,
    pub phone_number: String,
    pub car: Car,
}
