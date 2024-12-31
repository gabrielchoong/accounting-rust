# Project Documentation

## 1. Project Overview

**Project Name:** Accounting Application
**Description:**  
> This project is written for small accounting needs

**Tech Stack:**

- Rust
- External Crates: `serde`, `serde_json`

---

## 2. Dependencies

> Based on versions.

### **v0.1.0**

- serde: For serializing and deserializing the structs.
- serde_json: For handling JSON data conversion.

---

## 3. Next Steps & To-Do List

> Hammer away

- [ ] Refactor hardcoded values in create_details and create_invoice.
- [ ] Add error handling for invalid inputs.
- [ ] Optimize file append logic.

---

## 4. Version History

- v0.1.0: In development.

## 5. Change Log

> A timeline for my development progress

### **2024-12-27**

- Forked the code and created a new repository.
- Refactored the `InvoiceDetails` struct to `InvoiceItem` for better hierarchy.

### **2024-10-13**

- Added two core functions: `create_invoice`, `create_details` in `invoice_service.rs`.
- Added two util functions: `append_to_path`, `retrieve_json_data` in `json_util.rs`.

### **2024-10-12 and before**

- Created the project `accounting-rust`.
- Created four `struct` for storing core data: `car.rs`, `customer.rs`, `invoice.rs`, `invoice_details.rs`.
- Created a auth module for credentials.
