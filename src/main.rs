use serde_json::Value;
use futures::StreamExt;

mod database;

#[tokio::main]
async fn main() {

    database::init();

    let client = reqwest::Client::new();

    let auction_fetch = |page:u64| {
        let client = &client;
        async move {
            return match client.get("https://api.hypixel.net/skyblock/auctions")
                .query(&[("page",page)])
                .send()
                .await {
                    Err(e) => Err(e),
                    Ok(res) => match res.error_for_status() {
                        Err(e) => Err(e),
                        Ok(res) => Ok(res.json::<Value>().await.unwrap()),
                    },
                }
        }
    };

    let page0 = match auction_fetch(0).await {
        Err(e) => panic!("Something went wrong trying to fetch page 0 : {}",e),
        Ok(res) => res,
    };

    let j:u64 = page0["totalPages"].as_u64().unwrap();

    let stream = futures::stream::iter(1..j)
        .map(|page| {println!("started fetching page {}",page);auction_fetch(page)})
        .buffer_unordered(3);

    let stream = stream.then(|b| async { match b {
        Err(e) => panic!("Couldn't join task. This should not happen : {}",e),
        Ok(val) => val,
        }
    });

    let mut auction_contents:Vec<Value> = stream.collect::<Vec<Value>>().await;
    auction_contents.push(page0);

    database::append(&auction_contents);
}
