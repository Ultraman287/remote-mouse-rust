// This file implements a Rocket web server with functionality to control mouse movement and wait operations.
// It includes routes for serving a static HTML file, moving the mouse cursor, and pausing execution.

// External crates used in the application.

#[macro_use]
extern crate rocket;
use local_ip_address::list_afinet_netifas;
use local_ip_address::local_ip;
use mouse_rs::{types::keys::Keys, Mouse};
use rocket::fs::FileServer;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio::time::{sleep, Duration};
use std::fs;

// Struct representing a mouse movement with x and y coordinates.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MouseMove {
    x: i32,
    y: i32,
}

// Route to serve the index page.
#[get("/")]
fn index() -> content::RawHtml<String> {
    let contents = fs::read_to_string("static/hello.html").unwrap();
    let response = content::RawHtml(contents);
    response
}

// Asynchronous route to pause server execution for a given number of seconds.

#[get("/wait/<seconds>")]
async fn wait(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

// Route to move the mouse cursor based on the provided JSON data.
#[post("/move_mouse", format = "json", data = "<mouse_move>")]
fn move_mouse(mouse_move: Json<MouseMove>) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    mouse
        .move_to(pos.x + mouse_move.x, pos.y + mouse_move.y)
        .expect("Unable to move mouse");
}

// Function to update the Rocket.toml configuration file with a new IP address and port.

fn change_rocket_toml_ip(ip: String, port: String) {
    let contents = fs::read_to_string("example.toml").unwrap();
    let new_contents = contents.replace("localhost", &ip);
    let new_contents = new_contents.replace("8000", &port);
    fs::write("Rocket.toml", new_contents).unwrap();
}

// Entry point for the Rocket application.
#[launch]
fn rocket() -> _ {
    // Determines the local IP address and updates the Rocket.toml file accordingly.
    let mut local_ip: String;
    let network_interfaces = list_afinet_netifas().unwrap();
    for (name, ip) in network_interfaces.iter() {
        // println!("{}:\t{:?}, {}", name, ip, ip.is_ipv4());
        if ip.is_ipv4() && name == "Wi-Fi" {
            local_ip = ip.to_string();
            println!("local_ip: {}", local_ip);
            change_rocket_toml_ip(local_ip, "8000".to_string());
        }
    }
    // Configures and launches the Rocket web server with the defined routes.
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![wait])
        .mount("/", routes![move_mouse])
        .mount("/static", FileServer::from("static"))
}
