// send something to server
use core::panic;
use dotenv::dotenv;
use std::{collections::HashMap, error::Error};

pub struct Client {
    server_address: String,
    client: reqwest::blocking::Client,
}

pub fn run() {
    let client = Client::new();
    let _ = client.send_message();
}

// defining methods on client
impl Client {
    pub fn send_request(&self) -> Result<(), Box<dyn Error>> {
        let body = self.client.get(self.server_address.as_str()).send()?.text();

        println!("{:#?}", body);

        Ok(())
    }

    pub fn send_message(&self) -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::new();

        map.insert("surendra", "cool");
        map.insert("albert ", "lame");

        let res = match self
            .client
            .post(self.server_address.as_str())
            .json(&map)
            .send()
        {
            Ok(res) => res.text().unwrap(),
            Err(err) => panic!("Error: {}", err),
        };



        // .body(body)
        // .send()?;
        println!("{:#?}", res);

        Ok(())
    }

    pub fn new() -> Client {
        dotenv().ok();

        let server_address =
            std::env::var("SERVER_HOST_ADDRESS").expect("SERVER_HOST_ADDRESS not in .env file");

        let client = reqwest::blocking::Client::new();

        return Client {
            server_address,
            client,
        };
    }
}

