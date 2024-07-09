# Menu control

This is a simple user management system developed in Rust. The project allows you to create new users, list existing users, and manage user preferences. I found myself pondering about menu systems and their implementation. This curiosity, combined with my desire to start a new project, led to the creation of Menu Control.

## Key Features:

- Interactive command-line interface with colored output
- Data persistence using JSON
- Create and manage user profiles
- List existing users
- Customize user preferences

This project serves as a playground for exploring Rust's capabilities in creating interactive console applications.

## Dependencies

The project uses the following libraries:

- `crossterm` (0.27.0): For terminal manipulation
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
2. List profiles
3. Profile preferences
4. About
5. Exit

Select an option by typing the corresponding number and pressing Enter.

## Project Structure

- `main.rs`: Program entry point
- `menu.rs`: Main menu logic and option control
- `json.rs`: Functions for JSON file manipulation
- `profile.rs`: Definition of `profile` and `Configuration` structures

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
