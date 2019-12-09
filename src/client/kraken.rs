use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::client::Path;
use crate::consts::API_URL;
use crate::consts::API_VER;
use crate::types::Asset;
use crate::types::AssetPair;
use crate::types::BoxError;
use crate::types::Interval;
use crate::types::Map;
use crate::types::OHLCResponse;
use crate::types::Orders;
use crate::types::Response;
use crate::types::SpreadResponse;
use crate::types::TickerPair;
use crate::types::Time;
use crate::types::TradeResponse;

#[derive(Debug)]
#[repr(C)]
pub struct Kraken {
  client: Client,
}

impl Kraken {
  pub fn public() -> Self {
    Self {
      client: Client::new(),
    }
  }

  /// Get server time
  /// https://www.kraken.com/features/api#get-server-time
  pub async fn time(&self) -> Result<Response<Time>, BoxError> {
    self.get_public("Time").await
  }

  /// Get asset info
  /// https://www.kraken.com/features/api#get-asset-info
  pub async fn assets(&self, filter: &[&str]) -> Result<Response<Map<Asset>>, BoxError> {
    let mut path: Path = Path::new("Assets");

    path.add_slice("asset", filter);

    self.get_public(path.as_str()).await
  }

  /// Get tradable asset pairs
  /// https://www.kraken.com/features/api#get-tradable-pairs
  pub async fn pairs(&self, filter: &[&str]) -> Result<Response<Map<AssetPair>>, BoxError> {
    let mut path: Path = Path::new("AssetPairs");

    path.add_slice("pair", filter);

    self.get_public(path.as_str()).await
  }

  /// Get ticker information
  /// https://www.kraken.com/features/api#get-ticker-info
  pub async fn ticker(&self, filter: &[&str]) -> Result<Response<Map<TickerPair>>, BoxError> {
    // TODO: Proper error
    assert!(
      !filter.is_empty(),
      "Invalid Filter. You must request at least one pair."
    );

    let mut path: Path = Path::new("Ticker");

    path.add_slice("pair", filter);

    self.get_public(path.as_str()).await
  }

  /// Get OHLC data
  /// https://www.kraken.com/features/api#get-ohlc-data
  pub async fn ohlc(
    &self,
    pair: &str,
    interval: Interval,
    since: Option<u64>,
  ) -> Result<Response<OHLCResponse>, BoxError> {
    let mut path: Path = Path::new("OHLC");

    path.add("pair", pair);
    path.add("interval", (interval as u64).to_string());

    if let Some(since) = since {
      path.add("since", since.to_string());
    }

    self.get_public(path.as_str()).await
  }

  /// Get order book
  /// https://www.kraken.com/features/api#get-order-book
  pub async fn orders(
    &self,
    pair: &str,
    count: Option<u64>,
  ) -> Result<Response<Map<Orders>>, BoxError> {
    let mut path: Path = Path::new("Depth");

    path.add("pair", pair);

    if let Some(count) = count {
      path.add("count", count.to_string());
    }

    self.get_public(path.as_str()).await
  }

  /// Get recent trades
  /// https://www.kraken.com/features/api#get-recent-trades
  pub async fn trades(
    &self,
    pair: &str,
    since: Option<u64>,
  ) -> Result<Response<TradeResponse>, BoxError> {
    let mut path: Path = Path::new("Trades");

    path.add("pair", pair);

    if let Some(since) = since {
      path.add("since", since.to_string());
    }

    self.get_public(path.as_str()).await
  }

  /// Get recent spread data
  /// https://www.kraken.com/features/api#get-recent-spread-data
  pub async fn spread(
    &self,
    pair: &str,
    since: Option<u64>,
  ) -> Result<Response<SpreadResponse>, BoxError> {
    let mut path: Path = Path::new("Spread");

    path.add("pair", pair);

    if let Some(since) = since {
      path.add("since", since.to_string());
    }

    self.get_public(path.as_str()).await
  }

  async fn get_public<T: DeserializeOwned>(&self, path: &str) -> Result<Response<T>, BoxError> {
    Ok(self.get(&pub_url(path)).await?)
  }

  async fn get_private<T: DeserializeOwned>(&self, path: &str) -> Result<Response<T>, BoxError> {
    Ok(self.get(&prv_url(path)).await?)
  }

  async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<Response<T>, BoxError> {
    Ok(self.client.get(url).send().await?.json().await?)
  }
}

#[inline]
fn pub_url(path: &str) -> String {
  format!("https://{}/{}/public/{}", API_URL, API_VER, path)
}

#[inline]
fn prv_url(path: &str) -> String {
  format!("https://{}/{}/private/{}", API_URL, API_VER, path)
}
