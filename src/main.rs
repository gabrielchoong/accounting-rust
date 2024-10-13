mod auth;
mod utils;
mod models;
mod services;

use crate::models::invoice::Invoice;

use crate::services::invoice_service::InvoiceService;
use crate::utils::json_util::{append_to_file, retrieve_json_data};
use serde_json;

fn main() { 

    // auth::login_screen();

    let filepath = "data.json";

    let first_invoice = InvoiceService::create_invoice();

    let json_invoice = serde_json::to_string(&first_invoice).unwrap();

    let _ = append_to_file(filepath, &json_invoice);

    let Ok((data, content)) = retrieve_json_data::<Invoice>(filepath) else { panic!("You've hit an error!") };
}
