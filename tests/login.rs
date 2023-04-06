use std::process::Command;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::json;


fn reset_database() {
    let output = Command::new("diesel")
        .args(&["migration", "redo"])
        .output()
        .expect("failed to execute command");

    if output.status.success() {
        println!("Database migration successful");
    } else {
        println!(
            "Database migration failed with error:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn test_login() {
    // Setup
    reset_database();
    let client = Client::tracked(todo_rust::rocket()).unwrap();
    let mut response = client
        .post("/api/register")
        .header(ContentType::JSON)
        .body(json!({
            "username": "testuser", 
            "firstName": "Victor Eduardo", 
            "lastName": "Garcia Najera", 
            "password": "password" 
        }).to_string())
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Successfull
    response = client
        .post("/api/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "testuser",
            "password": "password"
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("authorizationToken"));

    // Invalid credentials
    response = client
        .post("/api/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "testuser",
            "password": "wrongpassword"
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.into_string().unwrap().contains("invalid credentials"));

    // No existent user
    response = client
        .post("/api/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "nonexistentuser",
            "password": "password"
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    assert!(response.into_string().unwrap().contains("invalid credentials"));
}
