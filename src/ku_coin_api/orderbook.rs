use std::fmt::{self, Display};

// contains 5 most recent bids and asks
pub struct Orderbook {
    asks: [(f64, u64); 5],
    bids: [(f64, u64); 5],
}

// toString method for printing orderbook
impl Display for Orderbook 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
         writeln!(f, "{:<10} {:<15}\t\t{:<10} {:<15}", "Ask Price", "Ask Size", "Bid Price", "Bid Size")?;
            
        // Ensure there are at least 5 asks and bids to prevent out-of-bounds access
        let len = self.asks.len().min(5).min(self.bids.len());
    
        for i in 0..len {
            writeln!(
                f,
                "{:<10.2} {:<15.2}\t\t{:<10} {:<15}",
                self.asks[i].0, self.asks[i].1, self.bids[i].0, self.bids[i].1
            )?;
        }
    
        Ok(())
    }
}

impl Orderbook {

    // returns 5 most recent values from passed in json
    fn update(msg: &serde_json::Value) -> [(f64, u64); 5] 
    {
        let data = msg
            .as_array()
            .expect("Expected an array")
            .iter()
            .map(|entry| {
                let price = entry
                    .get(0)
                    .expect("Error getting price")
                    .as_str()
                    .expect("Error converting to string");
                let price = price.parse::<f64>().expect("Error converting to float");
                let size = entry
                    .get(1)
                    .expect("Error getting size")
                    .as_u64()
                    .expect("Error converting to int");
                (price, size)
            });

        let mut ans = [(0.0, 0); 5];
        for (index, price) in data.enumerate() 
        {
            if index < 5 
            {
                ans[index] = price;
            } 
            else 
            {
                break;
            }
        }

        ans
    }

    // parses asks and bids into our orderbook
    pub fn get_book(data: serde_json::Value) -> Self
    {
        let asks = data.get("asks").expect("asks not found");
        let bids = data.get("bids").expect("asks not found");

        Orderbook {
            asks: Self::update(asks),
            bids: Self::update(bids),
        }
    }
}