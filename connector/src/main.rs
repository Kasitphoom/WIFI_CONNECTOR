// #![allow(dead_code)]

use wifi_rs::WiFi;

use wifi_rs::prelude::*;

use webbrowser;

fn connect_wifi() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect("Phoom's Galaxy S20 Ultra 5G", "phoom555") {
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

fn call_browser(){
    if webbrowser::open("https://www.google.com").is_ok(){
        println!("Web browser opened!");
    }
}


fn main() {
    connect_wifi();

    call_browser();
}

