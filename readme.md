# MySQL wrapper with Rust using SQLx

This project is an example of a simple repository pattern implementation in Rust for interacting with a MySQL database. It uses the `sqlx` library for database interaction and supports basic CRUD (Create, Read, Update, Delete) operations.

## Features

- **Fetch All Records**: Retrieve all records from a specified table with optional pagination (`LIMIT` and `OFFSET`).
- **Fetch One Record**: Retrieve a single record by its ID.
- **Insert Record**: Insert a new record into a specified table.
- **Update Record**: Update an existing record by its ID.
- **Delete Record**: Delete a record by its ID.

## Technologies Used

- **Rust**: The programming language used for the implementation.
- **SQLx**: An async, pure Rust SQL crate that provides support for various SQL databases, including MySQL.
- **MySQL**: The relational database used in this example.
- **dotenv**: To load environment variables from a `.env` file.
- **Tokio**: An asynchronous runtime for Rust, used to handle async operations.

## Project Structure

- **Product Struct**: Represents a product entity with fields like `id`, `name`, `price`, and `description`.
- **Repository Struct**: Encapsulates the database connection pool and provides methods for CRUD operations.
- **FetchError Enum**: Custom error types for handling various database-related errors.
- **Main Function**: Entry point of the application that demonstrates the usage of the repository methods.

## Setup and Usage

### Prerequisites

- Rust (latest version)
- MySQL database
- `.env` file with the following variable:

  ```
  DATABASE_URL=mysql://username:password@host:port/database
  ```

### Running the Project

1. **Clone the Repository**

   ```sh
   git clone https://github.com/andrewchuev/rust-db-wrapper.git
   cd rust-db-wrapper
   ```

2. **Install Dependencies**

   Make sure you have Rust installed, and then run:

   ```sh
   cargo build
   ```

3. **Set Up Environment Variables**

   Create a `.env` file in the root directory and add your MySQL database credentials as shown above.

4. **Run the Application**

   ```sh
   cargo run
   ```

### Example Usage

- **Fetch All Products**: Retrieves up to 10 products with pagination.
- **Fetch Product by ID**: Retrieves a product by its ID (e.g., ID 1).
- **Insert a New Product**: Inserts a new product with specified fields.
- **Update a Product**: Updates the name and price of a product by ID.
- **Delete a Product**: Deletes a product by its ID.

## Functions Overview

### `fetch_all<T>`
Fetches all records from the specified table, with optional `limit` and `offset` for pagination.

### `fetch_one<T>`
Fetches a single record from the specified table by `id`.

### `insert_record`
Inserts a new record into the specified table using the provided field values.

### `update_record`
Updates a record in the specified table by `id` with the provided field values.

### `delete_record`
Deletes a record from the specified table by `id`.

## Error Handling

The project uses the `thiserror` crate to define a custom `FetchError` enum to handle various errors that may occur during database interactions, such as:
- **QueryError**: Errors during database query execution.
- **NoRecordsFound**: No records were found in the specified table.
- **NoRecordFound**: No record found with the specified ID.

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for discussion.

## Contact

If you have any questions, feel free to open an issue or contact the repository maintainer.

---

This project serves as a foundation for learning Rust and SQLx and can be extended further for more complex operations or integrated with other services as needed.

