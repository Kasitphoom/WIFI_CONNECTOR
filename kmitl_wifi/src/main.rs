#![allow(dead_code)]

use reqwest::Client;
use wifi_rs::WiFi;
use wifi_rs::prelude::*;

use std::{thread, time};


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

    

    let values = [("password", ""), ("userName", "")];

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

    println!("\nWait for 5 seconds......");

    thread::sleep(time::Duration::from_secs(5));

    call_browser();

    Ok(())

}

