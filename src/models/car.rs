use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub id: i32,        
    pub model: String,
    pub registration_number: String,
}
