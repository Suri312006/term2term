// send something to server
use core::panic;
use dotenv::dotenv;
use std::{collections::HashMap, error::Error};

// TODO: rename this to something better later on
pub struct Args {
    pub message: String,

    pub recipient: String,
}

//TODO: we are creating a new client for pretty much every single new request
pub struct Client {
    server_address: String,
    client: reqwest::blocking::Client,
    args: Args,
}

// run fuction that contains library run code
pub fn run(args: Args) {
    let client = Client::new(args);
    let _ = client.test_connection();
    let _ = client.send_message();
}

impl Client {
    pub fn test_connection(&self) -> Result<(), Box<dyn Error>> {
        let res = match self.client.get(self.server_address.as_str()).send() {
            Ok(res) => res.text().unwrap(),
            Err(err) => panic!("Error: {}", err),
        };

        println!("{:#?}", res);

        Ok(())
    }
    // sends message to backend that sends args
    pub fn send_message(&self) -> Result<(), Box<dyn Error>> {
        let mut map = HashMap::new();

        map.insert(&self.args.recipient, &self.args.message);

        let res = match self
            .client
            .post(self.server_address.as_str())
            .json(&map)
            .send()
        {
            Ok(res) => res.text().unwrap(),
            Err(err) => panic!("Error: {}", err),
        };

        println!("{}", res);

        Ok(())
    }

    // new client
    pub fn new(args: Args) -> Client {
        dotenv().ok();

        let server_address =
            std::env::var("PUBLIC_SERVER_URL").expect("SERVER_HOST_ADDRESS not in .env file");

        let client = reqwest::blocking::Client::new();

        return Client {
            server_address,
            client,
            args,
        };
    }
}
