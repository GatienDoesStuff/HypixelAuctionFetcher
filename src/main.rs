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

	database::append(&page0);
	eprintln!("Fetched and appended page 0 of the auction house to the database. There are {} pages", j);

    let stream = futures::stream::iter(1..j)
        .map(|page| {eprintln!("started fetching page {}",page);auction_fetch(page)})
        .buffer_unordered(3);

    let _ = stream.for_each(|b| async { match b {
        Err(e) => panic!("Something went wrong : {}",e),
        Ok(val) => database::append(&val),
        }
    })
    .await;
}
