use client::Client;

fn main() {
    let client = Client::new();

    let _ = client.send_request();
}
