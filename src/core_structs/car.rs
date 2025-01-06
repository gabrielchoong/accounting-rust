use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Car {
    pub car_model: String,
    pub car_reg_number: String,
}
