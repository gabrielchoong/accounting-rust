use std::io;
use std::env;
use rpassword::read_password;

pub fn credential_check(io_password: String, io_username: String) {
    const ENVIRONMENT_VARIABLE: &str = "ACCOUNTING_APPLICATION";
    let password = env::var(ENVIRONMENT_VARIABLE)
        .expect("Password not set");

    const USERS_VARIABLE: &str = "ACCOUNTING_USER";
    let username = env::var(USERS_VARIABLE)
        .expect("User not set");

    let io_password = io_password.trim();
    let io_username = io_username.trim();

    // compare hashed passwords in prod
    if io_password == password && io_username == username {
        println!("Welcome, {io_username}.");
    } else {
        println!("Invalid username or password, please try again.");
    }
}

pub fn login_screen() {
    let mut username = String::new();

    println!("username:\t");
    io::stdin()
        .read_line(&mut username)
        .unwrap();

    println!("password:\t");
    let password = read_password()
        .unwrap();

    credential_check(password, username);
}

