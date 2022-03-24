# HypixelAuctionHelper

Fetches the Hypixel skyblock API and insters it into a SQLite database !

Current plans :
- Implement a clear function, removing outdated auctions from the database (those with outdated timestamps or found in `auctions_ended` provided by the API)
- Append to the dabase on a per-page basis, instead of appending every pages at once
- Data analysis, to guess the prices of an item (allowing to monitor items which are priced too low)
