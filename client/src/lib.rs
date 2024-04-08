// send something to server
use dotenv::dotenv;
use std::error::Error;

pub struct Client {
    server_address: String,
}

// defining methods on client
impl Client {

    pub fn send_request(&self) -> Result<(), Box<dyn Error>> {
        let body = reqwest::blocking::get(self.server_address.as_str())?.text();

        println!("{:#?}", body.unwrap());

        Ok(())
    }

    pub fn new() -> Client {
        dotenv().ok();

        let server_address =
            std::env::var("SERVER_HOST_ADDRESS").expect("SERVER_HOST_ADDRESS not in .env file");

        return Client { server_address };
    }
}
