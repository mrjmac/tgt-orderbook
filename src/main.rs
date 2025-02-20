use reqwest;
use serde_json::Value;
use tungstenite::{connect, Message};
use url::Url;

fn get_token() -> (String, String) 
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

    (token, endpoint)
}

fn main() 
{
    let (token, endpoint) = get_token();
    let url = format!("{}?token={}", endpoint, token);
    let (mut socket, _response) = connect(Url::parse(&url).expect("Invalid WebSocket URL").to_string())
        .expect("Failed to connect");
    
    let welcome_msg = socket.read().expect("Failed to read message");
    if let Message::Text(text) = welcome_msg {
        let json: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        assert_eq!(json["type"].as_str(), Some("welcome"));
        println!("{:?}", json);
    }
    
    let subscribe_msg = serde_json::json!({
        "id": 1,
        "type": "subscribe",
        "topic": "/contractMarket/level2Depth5:ETHUSDTM",
        "privateChannel": false,
        "response": false
    });
    
    socket.send(Message::Text(subscribe_msg.to_string().into()))
        .expect("Failed to send message");
    
    loop {
        let msg = socket.read().expect("Failed to read message");
        if let Message::Text(text) = msg {
            let json: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
            println!("{:?}", json);
        }
    }
}
