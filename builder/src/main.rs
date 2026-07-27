use std::fs;
use std::process::Command;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct OutbackConfig {

    server_type:String,

    version:String,

    jar:String,

}



fn main() {

    println!("================================");
    println!("       OUTBACK SERVER");
    println!("================================");


    let data = fs::read_to_string("outback-server.json")
        .expect("Missing outback-server.json");


    let config: OutbackConfig =
        serde_json::from_str(&data)
        .expect("Invalid config");


    println!(
        "Starting {} {}",
        config.server_type,
        config.version
    );


    match config.server_type.as_str() {


        "paper" => {

            start_server(
                &config.jar
            );

        },


        "forge" => {

            start_server(
                &config.jar
            );

        },


        "fabric" => {

            start_server(
                &config.jar
            );

        },


        "proxy" => {

            start_server(
                &config.jar
            );

        },


        _ => {

            println!(
                "Unknown server type"
            );

        }

    }

}



fn start_server(jar:&str){

    println!(
        "Launching {}",
        jar
    );


    Command::new("java")
        .args([
            "-Xms2G",
            "-Xmx4G",
            "-jar",
            jar,
            "nogui"
        ])
        .spawn()
        .expect("Failed to start server");

}