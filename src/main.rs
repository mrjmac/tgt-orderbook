mod ku_coin_api;
use crate::ku_coin_api::session::KuCoinSession;
use serde_json::Value;

// method to get token and timeout time from the api as outlined in 
// https://www.kucoin.com/docs/websocket/basic-info/apply-connect-token/public-token-no-authentication-required-

pub fn get_token() -> (String, u64)
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
    
    let timeout = res["data"]["instanceServers"][0]["pingInterval"]
        .as_u64()
        .expect("Missing or invalid pingInterval");

    // return token as string
    (format!("{}?token={}", endpoint, token), timeout)
}

fn main() 
{
    // get our token
    let (url, timeout) = get_token();
    // get a client connection
    let mut client = KuCoinSession::connect(&url);

    // subscribe our client to requested exchange
    client.subscribe("ETHUSDTM");

    loop 
    {
        let orderbook = client.update(timeout);
        println!("{}", orderbook)
    }
}
