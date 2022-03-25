# HypixelAuctionFetcher

Fetches the Hypixel skyblock API and inserts it into a SQLite database !

Current plans :
- Implement a clear function, removing outdated auctions from the database (those with outdated timestamps or found in `auctions_ended` provided by the API)
- Append to the dabase on a per-page basis, instead of appending every page at once
- Data analysis, to guess the prices of an item (allowing to monitor items which are priced too low)

# Building

`cargo build --release`

Output : `HypixelAuctionFetcher/target/release/skyblock_auction_fetcher`
