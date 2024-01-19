# Rust Mouse Control Web Server

This application is a Rust-based web server using the Rocket framework. It provides functionalities to control mouse movement and to execute wait operations, accessible through a web interface. This README outlines how to set up, run, and use the application.

## Prerequisites

Before you start, ensure you have the following installed:
- Rust and Cargo (Rust's package manager) - [Installation guide](https://www.rust-lang.org/tools/install)
- Access to a Wi-Fi network

## Installation

1. Clone the repository to your local machine:

```bash
git clone 
```

2. Navigate to the cloned repository's directory:

```bash
cd hello-rocket
```


## Running the Application

1. In the project directory, run the application using Cargo:

```bash
cargo run
```

2. The Rocket server will start and display the local IP address and port in the console.

## Connecting to the Server from a Phone

To control the mouse from your phone:
1. Connect your phone to the same Wi-Fi network as the computer running the application.
2. Open a web browser on your phone.
3. Enter the IP address and port displayed in the server's console. For example: `http://192.168.1.10:8000`
4. The web interface should load, allowing you to control the mouse and initiate wait operations from your phone.

## Additional Notes

- Ensure that your firewall settings allow incoming connections to the server.
- If you face any issues, check the console for error messages or logs for troubleshooting.

## Further Exploration

You can extend the application by adding more routes and functionalities, or by enhancing the front-end interface for a more interactive experience.

Thank you for using or contributing to this Rust Mouse Control Web Server application!
