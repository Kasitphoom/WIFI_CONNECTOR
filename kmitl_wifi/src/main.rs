#![allow(dead_code)]

use reqwest::Client;
use wifi_rs::WiFi;
use wifi_rs::prelude::*;

use std::fmt::format;
use std::io;
use std::io::{stdin, stdout, Write, Read};
use std::fs::File;


use reqwest;

// new connect wifi function without phrasing in values
fn connect_wifi_new() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);
    match wifi.connect("KMITL-WIFI", "") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successful."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }
    Ok(())
}

#[tokio::main]

async fn call_browser(){

    

    let values = [("password", "Phoom2547!!Kmitl"), ("userName", "65011328")];

    let client = reqwest::Client::new();

    let res = client.post("http://connect4.kmitl.ac.th:8080/PortalServer/Webauth/webAuthAction!login.action")
        .form(&values)
        .send()
        .await
        .unwrap();

    println!("{:?}", res);

}

fn main() -> Result<(), std::io::Error>{

    connect_wifi_new();

    call_browser();

    Ok(())

}
