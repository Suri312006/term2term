use dotenv::dotenv;
use std::error::Error;

fn main() {
    dotenv().ok();

    let server_address =
        std::env::var("SERVER_HOST_ADDRESS").expect("SERVER_HOST_ADDRESS not in .env file");

    let err = send_request(server_address.clone());
    println!("{:#?}", err);
}

fn send_request(address: String) -> Result<(), Box<dyn Error>> {
    // let res = reqwest::blocking::get("https://localhost:6969/")?.text()?;

    let body = reqwest::blocking::get(address)?.text();

    println!("{:#?}", body);

    Ok(())
}
