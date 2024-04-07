use std::error::Error;

fn main() {
    println!("Hello, world!");

    let err = send_request();
    println!("{:#?}", err);
}

fn send_request() -> Result<(), Box<dyn Error>> {
    // let res = reqwest::blocking::get("https://localhost:6969/")?.text()?;

    let body = reqwest::blocking::get("http://localhost:6969/")?.text();

    println!("{:#?}", body);

    Ok(())
}
