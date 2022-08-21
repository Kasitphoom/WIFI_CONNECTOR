#![allow(dead_code)]

use reqwest::Client;
use wifi_rs::WiFi;
use wifi_rs::prelude::*;

use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::{stdin, stdout, Write, Read};
use std::fs::File;
use std::os::windows::process;
use std::process::exit;

use reqwest;

fn connect_wifi(name: &str, password: &str) -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect(name, password) {
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

async fn aj_request(username: &str, password: &str){
    let values = [("password", &password), ("username", &username)];

    let client = reqwest::Client::new();

    let res = client.post("http://192.168.104.1/login?")
        .form(&values)
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
}


// make a function that check file exist or not
fn check_file_exist(file_name: &str) -> bool {
    match File::open(file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check_file() -> std::io::Result<()>{

    
    
    // if file not exist create new file
    if !check_file_exist("./impt.txt") {
        let mut f = File::create("./impt.txt")?;
        
        // get user input for username and password
        loop{

            println!("Please enter your prefered wifi name:");
            let mut wifi_name = String::new();
            io::stdin().read_line(&mut wifi_name)?;

            println!("Please enter your username:");
            let mut username = String::new();
            io::stdin().read_line(&mut username)?;
    
    
            println!("Please enter your password:");
            let mut password = String::new();
            io::stdin().read_line(&mut password)?;
            
    
            let contents = format!("{}\n{}\n{}", &wifi_name.trim(), &username.trim(), &password.trim());
    
            println!("\n\nwifi name: {} \nusername :{} \npassword :{}\n...........\nCorrect? [y/n]", &wifi_name, &username, &password);

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;
            if confirm.as_str().trim() == "y" {
                write!(&mut f, "{}", &contents)?;
                break
            }
        }
        
    }

    Ok(())

}


fn main() -> Result<(), std::io::Error>{

    // connect_wifi_new();

    // call_browser();

    check_file();

    // open file and read line
    let mut f = File::open("./impt.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut lines = contents.lines();
    let wifi_name = lines.next().unwrap();
    let username = lines.next().unwrap();
    let password = lines.next().unwrap();
    println!("{}", &wifi_name);
    println!("{}", &username);
    println!("{}", &password);

    connect_wifi(&wifi_name, "");

    aj_request(&username, &password);
    Ok(())

}

