# Simple CLI Weather App

A simple command-line weather application written in Rust that fetches and displays current weather information for any city using the OpenWeatherMap API.

## Features

- Fetch real-time weather data for any city worldwide
- Display temperature in both Celsius and Fahrenheit
- Simple and intuitive command-line interface
- Built with Rust for performance and reliability

## Prerequisites

- Rust (1.70 or higher)
- An OpenWeatherMap API key (free tier available at https://openweathermap.org/api)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Simple_CLI
```

2. Set up your OpenWeatherMap API key as an environment variable:
```bash
export RUSTY_WEATHER_API_KEY="your_api_key_here"
```

3. Build the project:
```bash
cargo build --release
```

## Usage

Run the application with a city name as an argument:

```bash
cargo run <city_name>
```

Or use the compiled binary:

```bash
./target/release/Simple_CLI <city_name>
```

### Examples

```bash
cargo run London
cargo run "New York"
cargo run Tokyo
```

### Output

The application will display the current temperature in both Celsius and Fahrenheit:

```
Temperature: 15.32C / 59.58F
```

## Dependencies

- `reqwest` (0.12.12) - HTTP client for making API requests
- `serde` (1.0) - Serialization/deserialization framework

## Project Structure

```
Simple_CLI/
├── main.rs       # Main application code
├── Cargo.toml    # Project configuration and dependencies
└── README.md     # This file
```

## Error Handling

The application will exit with an error message if:
- The `RUSTY_WEATHER_API_KEY` environment variable is not set
- No city name is provided as an argument
- The API request fails
- The city is not found

## License

This project is available for educational purposes.

## Contributing

This is a learning project. Feel free to fork and experiment!

## Future Improvements

Potential enhancements could include:
- Display additional weather information (humidity, wind speed, etc.)
- Add command-line argument parsing with `clap`
- Support for multiple output formats (JSON, CSV, etc.)
- Weather forecast for upcoming days
- Unit selection (metric/imperial) as a command-line option
