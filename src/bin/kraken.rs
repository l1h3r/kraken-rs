use std::error::Error;

use kraken::client::Kraken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let kraken: Kraken = Kraken::public();

  println!("time >");
  println!("{:#?}", kraken.time().await?);

  // println!("assets >");
  // println!("{:#?}", kraken.assets(&["XETH"]).await?);

  // println!("pairs >");
  // println!("{:#?}", kraken.pairs(&["XETHXXBT"]).await?);

  // println!("ticker >");
  // println!("{:#?}", kraken.ticker(&["XETHXXBT"]).await?);

  // println!("ohlc >");
  // println!("{:#?}", kraken.ohlc("XETHXXBT", kraken::types::Interval::M5, 0).await?);

  // println!("orders >");
  // println!("{:#?}", kraken.orders("XETHXXBT", 5).await?);

  // println!("trades >");
  // println!("{:#?}", kraken.trades("XETHXXBT", None).await?);

  // println!("spread >");
  // println!("{:#?}", kraken.spread("XETHXXBT", None).await?);

  Ok(())
}
