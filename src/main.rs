mod KuCoinAPI;
use crate::KuCoinAPI::session::KuCoinSession;
use serde_json::Value;

// method to get token from the api as outlined in 
// https://www.kucoin.com/docs/websocket/basic-info/apply-connect-token/public-token-no-authentication-required-

pub fn get_token() -> String
{
    let client = reqwest::blocking::Client::new();
    let res: Value = client
        .post("https://api.kucoin.com/api/v1/bullet-public")
        .send()
        .expect("Failed to send request")
        .json()
        .expect("Failed to parse JSON response");
        
    let token = res["data"]["token"]
        .as_str()
        .expect("Missing token")
        .to_string();

    let endpoint = res["data"]["instanceServers"][0]["endpoint"]
        .as_str()
        .expect("Missing endpoint")
        .to_string();

    // return as string
    format!("{}?token={}", endpoint, token)
}

fn main() 
{
    // get our token
    let url = get_token();
    // get a client connection
    let mut client = KuCoinSession::connect(&url);

    // subscribe our client to requested exchange
    client.subscribe("ETHUSDTM");

    loop 
    {
        // update client with error handling
        match client.update() {
            Ok(Some(order_book)) => println!("{}", order_book),
            Ok(None) => println!("No new order book update."),
            Err(err) => eprintln!("Error fetching order book: {}", err),
        }
    }
}
