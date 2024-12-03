// This file implements a Rocket web server with functionality to control mouse movement and wait operations.
// It includes routes for serving a static HTML file, moving the mouse cursor, and pausing execution.
// The application demonstrates the use of Rocket framework for building web applications in Rust,
// with the integration of external crates for mouse control and asynchronous task management.

// External crates used in the application.
#[macro_use]
extern crate rocket;
use local_ip_address::list_afinet_netifas; // For retrieving network interfaces and local IP address.
use local_ip_address::local_ip; // For retrieving the local IP address.
use mouse_rs::{types::keys::Keys, Mouse}; // For controlling mouse movements.
use rocket::fs::FileServer; // For serving static files.
use rocket::response::content; // For serving custom content types.
use rocket::serde::json::Json; // For handling JSON data in requests and responses.
use rocket::serde::Deserialize; // For deserializing JSON data into Rust structures.
use rocket::tokio::time::{sleep, Duration}; // For asynchronous sleep functionality.
use std::fs; // For file system operations.

// Struct representing a mouse movement with x and y coordinates.
// This struct is deserialized from incoming JSON data.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MouseMove {
    x: i32, // Change in x-coordinate for mouse movement.
    y: i32, // Change in y-coordinate for mouse movement.
}

// Route to serve the index page.
// Reads the contents of a static HTML file located in the "static" directory and returns it as raw HTML content.
#[get("/")]
fn index() -> content::RawHtml<String> {
    // Read the file content from "static/hello.html".
    let contents = fs::read_to_string("static/hello.html").unwrap();
    let response = content::RawHtml(contents); // Return the file content as raw HTML.
    response
}

// Asynchronous route to pause server execution for a given number of seconds.
// This demonstrates the use of async/await and allows the server to handle other requests during the wait.
#[get("/wait/<seconds>")]
async fn wait(seconds: u64) -> String {
    // Pause execution for the specified duration.
    sleep(Duration::from_secs(seconds)).await;
    // Return a response indicating the wait duration.
    format!("Waited for {} seconds", seconds)
}

// Route to move the mouse cursor based on the provided JSON data.
// It accepts a JSON payload containing x and y offsets and moves the mouse cursor accordingly.
#[post("/move_mouse", format = "json", data = "<mouse_move>")]
fn move_mouse(mouse_move: Json<MouseMove>) {
    let mouse = Mouse::new(); // Create a new instance of the Mouse controller.
    let pos = mouse.get_position().unwrap(); // Get the current mouse position.
    mouse
        .move_to(pos.x + mouse_move.x, pos.y + mouse_move.y) // Move the mouse to the new position.
        .expect("Unable to move mouse"); // Handle any errors during mouse movement.
}

// Function to update the Rocket.toml configuration file with a new IP address and port.
// This ensures the web server listens on the correct IP and port for the local machine.
fn change_rocket_toml_ip(ip: String, port: String) {
    let contents = fs::read_to_string("example.toml").unwrap(); // Read the configuration template file.
    let new_contents = contents.replace("localhost", &ip); // Replace "localhost" with the provided IP.
    let new_contents = new_contents.replace("8000", &port); // Replace the default port with the specified port.
    fs::write("Rocket.toml", new_contents).unwrap(); // Write the updated configuration to Rocket.toml.
}

// Entry point for the Rocket application.
// Configures and launches the Rocket web server with the defined routes and dynamic IP configuration.
#[launch]
fn rocket() -> _ {
    // Determines the local IP address of the machine to update the server's configuration.
    let mut local_ip: String;
    let network_interfaces = list_afinet_netifas().unwrap(); // List all available network interfaces.
    for (name, ip) in network_interfaces.iter() {
        if ip.is_ipv4() && name == "Wi-Fi" { // Check if the interface is IPv4 and named "Wi-Fi".
            local_ip = ip.to_string(); // Extract the IP address as a string.
            println!("local_ip: {}", local_ip); // Print the detected local IP address.
            change_rocket_toml_ip(local_ip, "8000".to_string()); // Update the Rocket.toml file with the IP and port.
        }
    }
    // Configure and launch the Rocket web server with the defined routes.
    rocket::build()
        .mount("/", routes![index]) // Mount the index route for serving the static HTML page.
        .mount("/", routes![wait]) // Mount the wait route for asynchronous pauses.
        .mount("/", routes![move_mouse]) // Mount the move_mouse route for mouse control.
        .mount("/static", FileServer::from("static")) // Serve static files from the "static" directory.
}
