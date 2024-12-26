use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub phone: String,
}

impl Customer {

    pub fn get_customer() -> Customer {

        // check customer based on id
        //
        // try retrieve_current_customer()
        // if it fails, then create_new_customer()
        
        Customer {
            id: 1,
            name: "Gabriel".to_string(),
            phone: "123-456-7890".to_string()
        }
    }

    pub fn create_new_customer() -> Customer {

        // called if customer is new



        Customer {
            id: 1,
            name: "Gabriel".to_string(),
            phone: "123-456-7890".to_string()
        }
    }

    pub fn retrieve_current_customer() -> Customer {

        // called if customer is not new

        Customer {
            id: 1,
            name: "Gabriel".to_string(),
            phone: "123-456-7890".to_string()
        }
    }
}
