# RocketApp

This project is a Rust application that utilizes Diesel for interacting with a PostgreSQL database and implements basic CRUD (Create, Read, Update, Delete) operations.

## Dependencies
1. Rust: <https://rustup.rs/> (ensure you have Rust and Cargo installed)
2. Diesel: <https://diesel.rs/> (for interacting with the database)
3. PostgreSQL: <https://www.postgresql.org/> (a relational database management system)
## Setting Up
### Clone the Repository:

`git clone https://github.com/thekalpeshchoudhary/rocket-app`  
`cd rocket-app`
### Install Dependencies:

`cargo build`  
#### Configuration
Database Connection:

The project uses a configuration file (.env) to store the database connection details. Update this file with your PostgreSQL connection information (host, port, username, password, and database name).   
Variable name: __DATABASE_URL__

### Running the Project
Once you've set up the dependencies and configured the database connection, you can run the project using:

`cargo run`
