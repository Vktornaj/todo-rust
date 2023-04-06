use rocket::{http::{ContentType, Status}, local::blocking::Client};
use serde_json::json;
use std::process::Command;


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
fn test_user_register() {
    reset_database();
    let rocket = todo_rust::rocket();
    let client = Client::tracked(rocket).unwrap();	

    let mut response = client
        .post("/api/register")
        .header(ContentType::JSON)
        .body(json!({
            "username": "vktornaj", 
            "firstName": "Victor Eduardo", 
            "lastName": "Garcia Najera", 
            "password": "Password123" 
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    
    // Test a duplicate username creation
    response = client
        .post("/api/register")
        .header(ContentType::JSON)
        .body(json!({
            "username": "vktornaj", 
            "firstName": "Victor Eduardo2", 
            "lastName": "Garcia Najera2", 
            "password": "Password123" 
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::NotAcceptable);
    assert_eq!(
        response.into_string(),
        Some("username already exist".to_string())
    );

    // TODO: Add test an invalid password creation
    // let invalid_user = json!({
    //     "username": "anotheruser",
    //     "firstName": "Anotheruser Name", 
    //     "lastName": "Anotheruser Lastname", 
    //     "password": ""
    // });
    // response = client
    //     .post("/api/register")
    //     .header(ContentType::JSON)
    //     .body(invalid_user.to_string())
    //     .dispatch();
    // assert_eq!(response.status(), Status::NotAcceptable);
    // assert_eq!(response.into_string(), Some("password error".to_string()));
}