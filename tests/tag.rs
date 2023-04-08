use std::process::Command;
use rocket::http::{ContentType, Status, Header};
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
fn test_tag() {
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
    // -Get Token
    response = client
        .post("/api/login")
        .header(ContentType::JSON)
        .body(json!({
            "username": "testuser",
            "password": "password"
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let token: todo_rust::routes::user::Token = response.into_json().unwrap();

    // -Create Todo
    response = client
        .post("/api/todo")
        .header(ContentType::JSON)
        .header(Header::new(
            "Authorization", 
            format!("{} {}", &token.token_type, &token.authorization_token)
        ))
        .body(json!({
            "title": "first",
            "description": "short",
            "status": 0
        }).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    
    // Successfull
    response = client
        .put(format!("/api/todo/{}/tag/{}", 1, "urgent"))
        .header(Header::new(
            "Authorization", 
            format!("{} {}", &token.token_type, &token.authorization_token)
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    
    response = client
        .get(format!("/api/todos/{}/{}", 0, 1))
        .header(Header::new(
            "Authorization", 
            format!("{} {}", &token.token_type, &token.authorization_token)
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let todos: Vec<todo_rust::models::todo::TodoJson> = response.into_json().unwrap();

    assert_eq!(todos[0].tags.len(), 1);

    response = client
        .put(format!("/api/todo/{}/tag/{}", 1, "urgent2"))
        .header(Header::new(
            "Authorization", 
            format!("{} {}", &token.token_type, &token.authorization_token)
        ))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    
    // TODO: strengthen this test
    // Repeated tag
    for _ in 0..10 {
        response = client
            .put(format!("/api/todo/{}/tag/{}", 1, "urgent"))
            .header(Header::new(
                "Authorization", 
                format!("{} {}", &token.token_type, &token.authorization_token)
            ))
            .dispatch();

        assert_eq!(response.status(), Status::Conflict);
        
        response = client
            .put(format!("/api/todo/{}/tag/{}", 1, "urgent2"))
            .header(Header::new(
                "Authorization", 
                format!("{} {}", &token.token_type, &token.authorization_token)
            ))
            .dispatch();

        assert_eq!(response.status(), Status::Conflict);

        response = client
            .get(format!("/api/todos/{}/{}", 0, 1))
            .header(Header::new(
                "Authorization", 
                format!("{} {}", &token.token_type, &token.authorization_token)
            ))
            .dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        let todos: Vec<todo_rust::models::todo::TodoJson> = response.into_json().unwrap();
        
        assert_eq!(todos[0].tags.len(), 2);
    }

}
