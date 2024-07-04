# User Management System

This is a simple user management system developed in Rust. The project allows you to create new users, list existing users, and manage user preferences.

## Features

- Create new users
- List existing users
- Manage user preferences
- Store user data in JSON format

## Requirements

- Rust (2021 edition)
- Cargo

## Dependencies

The project uses the following libraries:

- `colored` (2.1.0): For coloring console output
- `serde_json` (1.0.118): For JSON serialization and deserialization
- `serde` (1.0): For deriving serialization/deserialization implementations

## Installation

1. Clone the repository:
   ```markdown
   git clone
   ```

2. Navigate to the project directory:
   ```markdown
   cd menu-control
   ```

3. Compile and run the project:
   ```markdown
   cargo run
   ```

## Usage

When you run the program, you'll see a menu with the following options:

1. Create new user
2. List users
3. User preferences
4. About
5. Exit

Select an option by typing the corresponding number and pressing Enter.

## Project Structure

- `main.rs`: Program entry point
- `menu.rs`: Main menu logic and option control
- `json.rs`: Functions for JSON file manipulation
- `user.rs`: Definition of `User` and `Configuration` structures

## Todo List

- [ ] Implement user preferences menu
- [X] Add functionality to list all users
- [X] Implement user data persistence
- [X] Add error handling for invalid user inputs
- [ ] Add unit tests for core functionality
- [ ] Improve documentation for each module
- [ ] Improve interface render

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to discuss improvements or report bugs.

## Author

Rafael Ramos - rafael.ramosrc@gmail.com

## License

MIT license
