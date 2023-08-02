## Rust Todo API

This is a simple Todo API written in Rust using the rocket framework for web and diesel framework for data. The API allows you to manage your todo tasks by performing basic CRUD (Create, Read, Update, Delete) operations on them.

### Getting Started

To run the Todo API, follow the steps below:

1. **Prerequisites**: Make sure you have Rust and Cargo installed on your system. You can install them from the official Rust website: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).

2. **Clone the repository**: Clone this repository to your local machine.

3. **Build the project**: Open a terminal, navigate to the project directory, and run the following command to build the project and its dependencies:

   ```bash
   cargo build
   ```

4. **Run the API**: After building the project, you can start the API using the following command:

   ```bash
   cargo run
   ```

   The API will be accessible at `http://127.0.0.1:8000`.

### API Endpoints

The Todo API exposes the following endpoints:

1. **Register User**

   - **URL**: `/api/register`
   - **Method**: POST
   - **Description**: Create a new user.
   - **Request Body**:
     ```json
     {
       "username": "sample_username",
       "password": "sample_password"
     }
     ```
   - **Response**: Returns the newly created user in JSON format.

2. **Check Username Availability**

   - **URL**: `/api/username-availability/<username>`
   - **Method**: GET
   - **Description**: Check if a username is available for registration.
   - **Response**: Returns a JSON object with the availability status.
     ```json
     {
       "isAvailable": true
     }
     ```

3. **Get User Info**

   - **URL**: `/api/user/info`
   - **Method**: GET
   - **Description**: Get user information by providing a valid authentication token.
   - **Request Header**: `Authorization: Bearer <token>`
   - **Response**: Returns user information in JSON format.

4. **Login**

   - **URL**: `/api/login`
   - **Method**: POST
   - **Description**: Authenticate the user and obtain an access token.
   - **Request Body**:
     ```json
     {
       "username": "sample_username",
       "password": "sample_password"
     }
     ```
   - **Response**: Returns an access token in JSON format.
     ```json
     {
       "authorization_token": "sample_access_token",
       "token_type": "Bearer"
     }
     ```

### Error Handling

The API handles errors gracefully and returns appropriate HTTP status codes along with error messages when necessary.

### Data Persistence

The API uses a database connection (`Db`) to interact with the data store for user management.

### Authentication and Authorization

This API includes basic authentication and authorization mechanisms using access tokens. Users can register, log in, and access specific endpoints using their access tokens.

### Contributions

Contributions to this project are welcome. Feel free to submit issues, suggestions, or pull requests on the project repository.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
