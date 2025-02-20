use serde_json::{json, Value};
use tungstenite::{WebSocket, connect, Message, stream::MaybeTlsStream};
use std::net::TcpStream;
use url::Url;
use crate::KuCoinAPI::orderbook::orderbook;

// basic websocket
pub struct KuCoinSession {
    sk: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl KuCoinSession
{
    // return a connection instance to the user
    // connects using api key and waits for the welcome message
    pub fn connect(token: &str) -> Self
    {
        let (mut socket, _response) = connect(Url::parse(&token).expect("Invalid WebSocket URL").to_string())
            .expect("Failed to connect");

        let welcome_msg = socket.read().expect("Failed to read message");
        if let Message::Text(text) = welcome_msg {
            let json: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
            assert_eq!(json["type"].as_str(), Some("welcome"));
            println!("Client received welcome message!");
        }

        Self{sk: socket}
    }

    // subscribes us to a specific exchange as outlined in
    // https://www.kucoin.com/docs/websocket/basic-info/subscribe/introduction
    pub fn subscribe(&mut self, symbol: &str)
    {
        let msg = json!({
            "id": 1,
            "type": "subscribe",
            "topic": format!("/contractMarket/level2Depth5:{}", symbol),
            "privateChannel": false,
            "response": true
        });

        self.sk.send(Message::Text(msg.to_string().into()))
        .expect("Failed to send message");
    }

    // pings the server to keep the connection alive and then waits for an orderbook update
    // could probably handle receiving pongs and sending pings using the timeout function in the api in a thread,
    // but seems really annoying to implement
    // TODO: make this not have a loop in it lol
    pub fn update(&mut self) -> orderbook
    {
        let ping_msg = json!({ "id": 1, "type": "ping" });

        self.sk.send(Message::Text(ping_msg.to_string().into()))
            .expect("Failed to send ping");

        loop
        {
            let msg_str = self.sk.read().expect("Failed to read message");
            if let Message::Text(text) = msg_str 
            {
                let msg: Value = serde_json::from_str(&text).expect("Failed to parse message");

                if msg["type"] == "message" && msg["data"].is_object() 
                {
                    let data = &msg["data"];

                    if data.get("asks").is_some() && data.get("bids").is_some() 
                    {
                        return orderbook::getBook(data.clone());
                    } 
                }
            }
        }
    }
}