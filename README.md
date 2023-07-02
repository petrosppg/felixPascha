# Rust Easter Date Calculator

This is a Rust program that calculates the Western and Orthodox Easter dates for a given year. The Western Easter date is calculated using the Meeus/Jones/Butcher algorithm, while the Orthodox Easter date is calculated using different Meeus algorithms for the Julian and Gregorian calendars.

## Prerequisites

- Rust programming language (stable version)

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/petrosppg/felixPascha.git
   ```

2. Navigate to the project directory:

   ```bash
   cd felixPascha
   ```

3. Open the `main.rs` file and replace the `year` variable in the `main` function with the desired year:

   ```rust
   let year = 2024; // Replace this with the year you want to calculate Easter for
   ```

4. Run the program:

   ```bash
   rustc main.rs
   ```

   The program will output the Western Easter date, followed by the Orthodox Easter dates in both the Julian and Gregorian calendars.

## Contributing

Contributions to the Rust Easter Date Calculator project are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.